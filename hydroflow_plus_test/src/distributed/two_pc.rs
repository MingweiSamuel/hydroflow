use std::hash::Hash;

use hydroflow_plus::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use stageleft::*;

#[derive(Serialize, Deserialize)]
struct SendOverNetwork {
    pub n: u32,
}

pub struct Leader {}
pub struct Follower {}

pub fn two_pc<'a, Key>(
    flow: &FlowBuilder<'a>,
    leader: &Process<Leader>,
    vote_requests: Stream<'a, Key, Unbounded, NoTick, Process<Leader>>,
    followers: &Cluster<Follower>,
    // vote_choice_send
) where
    Key: Clone + Serialize + DeserializeOwned + Eq + Hash,
{
    let cluster_members = flow.cluster_members(followers);
    let x: Stream<'_, usize, Unbounded, NoTick, Process<Leader>> = vote_requests
        .broadcast_bincode(&followers)
        .map(q!(|_vr| true))
        .send_bincode(leader)
        .tick_batch()
        .persist()
        .reduce_keyed(q!(|prev, new| {
            *prev |= new;
        }))
        .filter(q!(|&(_id, vote)| vote))
        .count()
        .filter(q!(|&c| c >= cluster_members.len()))
        .delta()
        .all_ticks();
    x.for_each(q!(|x| println!("{}", x)));

    // tick_cycle()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::join_all;
    use futures::SinkExt;
    use hydro_deploy::{Deployment, Host};
    use hydroflow_plus_deploy::{DeployCrateWrapper, TrybuildHost};

    #[tokio::test]
    async fn test_two_pc() {
        let mut deployment = Deployment::new();

        let flow = hydroflow_plus::FlowBuilder::new();
        let leader = flow.process();
        let followers = flow.cluster();
        let driver = flow.external_process::<()>();

        let (vote_req_send, vot_req_recv) = flow.source_external_bincode(&driver, &leader);
        super::two_pc(&flow, &leader, vot_req_recv, &followers);

        // let built = flow.with_default_optimize();
        // insta::assert_debug_snapshot!(built.ir());

        // if we drop this, we drop the references to the deployment nodes
        let nodes = flow
            .with_default_optimize()
            .with_process(&leader, TrybuildHost::new(deployment.Localhost()))
            .with_cluster(
                &followers,
                vec![TrybuildHost::new(deployment.Localhost()); 8],
            )
            .with_external(&driver, deployment.Localhost() as Arc<dyn Host>)
            .deploy(&mut deployment);

        deployment.deploy().await.unwrap();

        let mut vote_req_send = nodes.connect_sink_bincode(vote_req_send).await;
        vote_req_send.send("hello".to_owned()).await.unwrap();

        // let cluster = nodes.get_cluster(&followers);
        // let follower_stdouts = join_all(cluster.members().iter().map(|m| m.stdout())).await;

        let mut leader_stdout = nodes.get_process(&leader).stdout().await;

        deployment.start().await.unwrap();

        // for mut stdout in follower_stdouts {
        //     assert_eq!(stdout.recv().await.unwrap(), "\"hello\"");
        // }
        while let Some(msg) = leader_stdout.recv().await {
            eprintln!("{}", msg);
        }
    }
}
