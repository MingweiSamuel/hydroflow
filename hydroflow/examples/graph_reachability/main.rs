use hydroflow::builder::prelude::*;
use hydroflow::scheduled::handoff::VecHandoff;

pub fn main() {
    let mut builder = HydroflowBuilder::default();

    let (reachable_send, reachable_recv) = builder.make_edge::<VecHandoff<usize>, _>();

    let (edges_send, edges_recv) = builder.add_channel_input::<_, VecHandoff<(usize, usize)>>();

    let initial = builder.start_iter([0]);

    builder.add_subgraph(
        reachable_recv
            .flatten()
            .chain(initial)
            .map(|v| (v, ()))
            .join(edges_recv.flatten())
            .map(|(_v_old, (), v_new)| v_new)
            .pull_to_push()
            .tee(
                builder.start_tee().map(Some).push_to(reachable_send),
                builder.start_tee().for_each(|x| println!("Reached: {}", x)),
            ),
    );

    let mut hydroflow = builder.build();

    edges_send.give(Some((0, 3)));
    edges_send.give(Some((5, 10)));
    edges_send.give(Some((2, 3)));
    edges_send.flush();
    hydroflow.tick();

    println!("A");

    edges_send.give(Some((3, 5)));
    edges_send.flush();
    hydroflow.tick();

    println!("B");

    edges_send.give(Some((10, 2)));
    edges_send.flush();
    hydroflow.tick();
}
