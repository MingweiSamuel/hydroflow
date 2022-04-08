use hydroflow::tokio::{
    self,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::common::{Clock, Message};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct ActorId(u64);

#[derive(Clone)]
pub struct Kvs<K, V>
where
    K: Send + Clone,
    V: Send + Clone,
{
    senders: Vec<Sender<Message<K, V>>>,
    round_robin: usize,
}

impl<K, V> Kvs<K, V>
where
    K: 'static + Clone + Eq + std::hash::Hash + Send + std::fmt::Debug,
    V: 'static + Clone + Send + std::fmt::Debug + Ord + Default,
{
    pub fn new(workers: u64) -> Self {
        let senders = spawn_threads::<K, V>(workers);

        Kvs {
            senders,
            round_robin: 0,
        }
    }

    pub async fn set(&mut self, k: K, v: V) {
        let receiver = self.round_robin % self.senders.len();
        self.round_robin += 1;
        self.senders[receiver]
            .send(Message::Set(k, v))
            .await
            .unwrap();
    }

    pub async fn get(&mut self, k: K) -> Option<(Clock, V)> {
        // TODO: We need to make sure we talk to one that is correct, but for
        // now since everyone owns everything just send a message to whoever.
        let receiver_idx = self.round_robin % self.senders.len();
        self.round_robin += 1;
        let (sender, receiver) = futures::channel::oneshot::channel();
        self.senders[receiver_idx]
            .send(Message::Get(k, sender))
            .await
            .unwrap();

        receiver.await.ok()
    }
}

type Matrix<K, V> = Vec<(Receiver<Message<K, V>>, Vec<Sender<Message<K, V>>>)>;
type MessageSender<K, V> = Sender<Message<K, V>>;

fn make_communication_matrix<K, V>(n: u64) -> (Matrix<K, V>, Vec<MessageSender<K, V>>)
where
    K: Send + Clone,
    V: Send + Clone,
{
    let mut receivers = Vec::new();
    let mut senders: Vec<_> = (0..n).map(|_| Vec::new()).collect();
    let mut extra_senders = Vec::new();
    for _ in 0..n {
        let (sender, receiver) = channel(1024);
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
    V: 'static + Send + Clone,
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
        move |_, mut receiver: Receiver<Message<K, V>>, _| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                let event_loop = tokio::spawn(async move {
                    while let Some(msg) = receiver.recv().await {
                        // Do nothing.
                        let _ = msg;
                    }
                });

                event_loop.await.unwrap();
            })
        },
    )
}
