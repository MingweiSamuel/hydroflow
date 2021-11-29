//! Organizational module for Hydroflow Send/RecvCtx structs and Input/OutputPort structs.

use ref_cast::RefCast;
use taskpool::StateHandle;
use taskpool::slotmap::Key;

use super::handoff::{CanReceive, Handoff, TryCanReceive};
use super::HandoffData;


/**
 * Context provided to a compiled component for reading from an [InputPort].
 */
#[derive(RefCast)]
#[repr(transparent)]
pub struct RecvCtx<H: Handoff> {
    pub(crate) handoff: H,
}
impl<H: Handoff> RecvCtx<H> {
    pub fn take_inner(&self) -> H::Inner {
        self.handoff.take_inner()
    }
}
/**
 * Context provided to a compiled component for writing to an [OutputPort].
 */
#[derive(RefCast)]
#[repr(transparent)]
pub struct SendCtx<H: Handoff> {
    pub(crate) handoff: H,
}
impl<H: Handoff> SendCtx<H> {
    // // TODO: represent backpressure in this return value.
    // #[allow(clippy::result_unit_err)]
    // pub fn give(self, item: H::Item) -> Result<(), ()> {
    //     (*self.once.get()).borrow_mut().try_give(item)
    // }
    pub fn give<T>(&self, item: T) -> T
    where
        H: CanReceive<T>,
    {
        <H as CanReceive<T>>::give(&self.handoff, item)
    }

    pub fn try_give<T>(&self, item: T) -> Result<T, T>
    where
        H: TryCanReceive<T>,
    {
        <H as TryCanReceive<T>>::try_give(&self.handoff, item)
    }
}




/**
 * Handle corresponding to a [RecvCtx]. Consumed by [crate::scheduled::Hydroflow::add_edge] to construct the Hydroflow graph.
 */
#[must_use]
pub struct InputPort<H, Sid, Tid>
where
    H: Handoff,
    Sid: Key,
    Tid: Key,
{
    pub(crate) subgraph: Tid,
    pub(crate) handle: StateHandle<Option<StateHandle<HandoffData<H, Tid>, Sid>>, Sid>,
}
/**
 * Handle corresponding to a [SendCtx]. Consumed by [crate::scheduled::Hydroflow::add_edge] to construct the Hydroflow graph.
 */
#[must_use]
pub struct OutputPort<H, Sid, Tid>
where
    H: Handoff,
    Sid: Key,
    Tid: Key,
{
    pub(crate) subgraph: Tid,
    pub(crate) handle: StateHandle<Option<StateHandle<HandoffData<H, Tid>, Sid>>, Sid>,
}
// impl<T: Clone> Clone for OutputPort<TeeingHandoff<T>> {
//     fn clone(&self) -> Self {
//         Self {
//             sg_id: self.sg_id,
//             handoff_id: Rc::new(RefCell::new(self.handoff.borrow().tee())),
//         }
//     }
// }
