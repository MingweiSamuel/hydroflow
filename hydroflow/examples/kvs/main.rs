#![feature(never_type)]

extern crate hdrhistogram;

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

use futures::channel::oneshot::Canceled;
use hdrhistogram::Histogram;
use hydroflow::{
    builder::{
        prelude::{BaseSurface, PullSurface, PushSurface},
        HydroflowBuilder,
    },
    lang::{
        lattice::{dom_pair::DomPairRepr, map_union::MapUnionRepr, ord::MaxRepr},
        tag,
    },
    scheduled::handoff::VecHandoff,
    tokio::{
        self,
        sync::mpsc::{channel, Receiver, Sender},
    },
};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct ActorId(u64);

type Clock = HashMap<usize, u64>;

#[derive(Debug)]
enum Message<K, V>
where
    K: Send + Clone,
    V: Send + Clone,
{
    // A KV set request from a client.
    Set(K, V),
    // A KV get request from a client.
    Get(K, futures::channel::oneshot::Sender<(Clock, V)>),
    // A set of data that I am responsible for, sent to me by another worker.
    Batch((usize, u64), Vec<(K, V)>),
}

unsafe impl<K, V> Send for Message<K, V>
where
    K: Send + Clone,
    V: Send + Clone,
{
}

fn main() {
    let workers = 1;
    let kvs = Arc::new(Kvs::new(workers));
    let mut i = 0;
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut h = Histogram::<u64>::new(2).unwrap();
    rt.block_on(async move {
        loop {
            // tokio::time::sleep(Duration::from_millis(10)).await;
            let before = Instant::now();
            kvs.clone()
                .set(format!("foo{}", i), format!("bar{}", i))
                .await;
            h.record(before.elapsed().as_micros().try_into().unwrap())
                .unwrap();

            i += 1;
            if i % 100 == 0 {
                let k = format!("foo{}", i % 99);
                println!("doing a read for {}", k);
                let _ = kvs.clone().get(k).await;
                println!("read returned");
            }
        }
    });
}

struct Kvs<K, V>
where
    K: Send + Clone,
    V: Send + Clone,
{
    senders: Vec<Sender<Message<K, V>>>,
    round_robin: AtomicUsize,
}

impl<K, V> Kvs<K, V>
where
    K: 'static + Clone + Eq + std::hash::Hash + Send + std::fmt::Debug,
    V: 'static + Clone + Send + std::fmt::Debug + Ord + Default,
{
    fn new(workers: u64) -> Self {
        let senders = spawn_threads::<K, V>(workers);

        Kvs {
            senders,
            round_robin: AtomicUsize::new(0),
        }
    }

    async fn set(self: Arc<Self>, k: K, v: V) {
        let receiver = self.round_robin.fetch_add(1, Ordering::SeqCst) % self.senders.len();
        self.senders[receiver]
            .send(Message::Set(k, v))
            .await
            .unwrap();
    }

    async fn get(self: Arc<Self>, k: K) -> Result<(Clock, V), Canceled> {
        // TODO: We need to make sure we talk to one that is correct, but for
        // now since everyone owns everything just send a message to whoever.
        let receiver_idx = self.round_robin.fetch_add(1, Ordering::SeqCst) % self.senders.len();
        let (sender, receiver) = futures::channel::oneshot::channel();
        self.senders[receiver_idx]
            .send(Message::Get(k, sender))
            .await
            .unwrap();

        receiver.await
    }
}

type Matrix<K, V> = Vec<(Receiver<Message<K, V>>, Vec<Sender<Message<K, V>>>)>;
type MessageSender<K, V> = Sender<Message<K, V>>;

fn make_communication_matrix<K, V>(n: u64) -> (Matrix<K, V>, Vec<MessageSender<K, V>>)
where
    K: Send + Clone,
    V: Ord + Send + Clone,
{
    let mut receivers = Vec::new();
    let mut senders: Vec<_> = (0..n).map(|_| Vec::new()).collect();
    let mut extra_senders = Vec::new();
    for _ in 0..n {
        let (sender, receiver) = channel(8192);
        receivers.push(receiver);
        for s in senders.iter_mut() {
            s.push(sender.clone())
        }
        extra_senders.push(sender);
    }

    (
        receivers.into_iter().zip(senders.into_iter()).collect(),
        extra_senders,
    )
}

fn spawn<F, K, V>(n: u64, f: F) -> Vec<Sender<Message<K, V>>>
where
    F: 'static + Fn(usize, Receiver<Message<K, V>>, Vec<Sender<Message<K, V>>>) + Send + Clone,
    K: 'static + Send + Clone,
    V: 'static + Ord + Send + Clone,
{
    let (matrix, senders) = make_communication_matrix(n);
    for (i, (receiver, senders)) in matrix.into_iter().enumerate() {
        let f = f.clone();
        std::thread::spawn(move || f(i, receiver, senders));
    }

    senders
}

fn spawn_threads<K, V>(workers: u64) -> Vec<Sender<Message<K, V>>>
where
    K: 'static + Clone + Eq + std::hash::Hash + Send + std::fmt::Debug,
    V: 'static + Clone + Send + std::fmt::Debug + Ord + Default,
{
    spawn(
        workers,
        move |_id, receiver: Receiver<Message<K, V>>, _senders: Vec<Sender<Message<K, V>>>| {
            let mut hf = HydroflowBuilder::default();

            // let (q_send, q_recv) = hf
            //     .add_channel_input::<_, Option<Message<K, V>>, VecHandoff<Message<K, V>>>(
            //         "writes",
            //     );

            // tokio::spawn(async move {
            //     // TODO(justin): batch all these per try_recv?
            //     while let Some(v) = receiver.recv().await {
            //         q_send.give(Some(v));
            //         q_send.flush();
            //     }
            // });

            let q_recv = hf.add_input_from_stream::<_, Option<_>, VecHandoff<_>, _>(
                "incoming_messages",
                ReceiverStream::new(receiver).map(Some),
            );

            let (reads_send, reads_recv) = hf.make_edge::<_, VecHandoff<(
                K,
                futures::channel::oneshot::Sender<(Clock, V)>,
            )>, Option<(
                K,
                futures::channel::oneshot::Sender<(Clock, V)>,
            )>>("reads");

            hf.add_subgraph(
                "demultiplexer",
                q_recv
                    .flatten()
                    .pull_to_push()
                    .filter(|x| matches!(x, Message::Get(_, _)))
                    .map(|msg| {
                        if let Message::Get(k, sender) = msg {
                            Some((k, sender))
                        } else {
                            panic!()
                        }
                    })
                    .push_to(reads_send),
            );

            hf.add_subgraph(
                "read_handler",
                reads_recv.flatten().pull_to_push().for_each(|(_k, ch)| {
                    ch.send(Default::default()).unwrap();
                }),
            );

            hf.build().run().unwrap();
        },
    )
}
