use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::WeakUnboundedSender;

use super::SubgraphId;

/**
 * A handle into a specific [super::graph::Hydroflow] instance for triggering
 * subgraphs to run, possibly from another thread.
 */
#[derive(Clone)]
pub struct Reactor {
    pub(crate) event_queue_send: WeakUnboundedSender<SubgraphId>,
}
impl Reactor {
    pub fn trigger(&self, sg_id: SubgraphId) -> Result<(), Option<SendError<SubgraphId>>> {
        self.event_queue_send
            .upgrade()
            .ok_or(None)?
            .send(sg_id)
            .map_err(Some)
    }

    #[cfg(feature = "async")]
    pub fn into_waker(self, sg_id: SubgraphId) -> std::task::Waker {
        use futures::task::ArcWake;
        use std::sync::Arc;

        struct ReactorWaker {
            reactor: Reactor,
            sg_id: SubgraphId,
        }
        impl ArcWake for ReactorWaker {
            fn wake_by_ref(arc_self: &Arc<Self>) {
                arc_self.reactor.trigger(arc_self.sg_id).unwrap(/* TODO(mingwei) */);
            }
        }

        let reactor_waker = ReactorWaker {
            reactor: self,
            sg_id,
        };
        futures::task::waker(Arc::new(reactor_waker))
    }
}
