//! Organizational module for Hydroflow Send/RecvCtx structs and Input/OutputPort structs.
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;

use crate::scheduled::handoff::{CanReceive, Handoff, TryCanReceive};
use crate::scheduled::state::StateHandle;
use crate::scheduled::{Context, SubgraphId};

#[allow(type_alias_bounds)]
pub type HandoffStateHandle<H: Handoff> = StateHandle<RefCell<H::State>>;

/**
 * Context provided to a compiled component for writing to an [OutputPort].
 */
pub struct SendCtx<'a, H>
where
    H: Handoff,
    H::State: 'static,
{
    pub(crate) state_handle: StateHandle<RefCell<H::State>>,
    pub(crate) context: &'a Context<'a>,
    pub(crate) _phantom: PhantomData<&'a mut H>,
}
impl<'a, H> Clone for SendCtx<'a, H>
where
    H: Handoff,
    H::State: 'static,
{
    fn clone(&self) -> Self {
        Self {
            state_handle: self.state_handle,
            context: self.context,
            _phantom: PhantomData,
        }
    }
}
impl<'a, H> Copy for SendCtx<'a, H>
where
    H: Handoff,
    H::State: 'static,
{
}
impl<'a, H> SendCtx<'a, H>
where
    H: Handoff,
    H::State: 'static,
{
    // // TODO: represent backpressure in this return value.
    // #[allow(clippy::result_unit_err)]
    // pub fn give(self, item: H::Item) -> Result<(), ()> {
    //     (*self.once.get()).borrow_mut().try_give(item)
    // }
    pub fn give<T>(&self, item: T) -> T
    where
        H: CanReceive<T>,
    {
        let mut state = self.context.state_ref(self.state_handle).borrow_mut();
        <H as CanReceive<T>>::give(&mut *state, item)
    }

    pub fn try_give<T>(&self, item: T) -> Result<T, T>
    where
        H: TryCanReceive<T>,
    {
        let mut state = self.context.state_ref(self.state_handle).borrow_mut();
        <H as TryCanReceive<T>>::try_give(&mut *state, item)
    }
}

/**
 * Handle corresponding to a [SendCtx]. Consumed by [crate::scheduled::Hydroflow::add_edge] to construct the Hydroflow graph.
 */
#[must_use]
pub struct OutputPort<H>
where
    H: Handoff,
{
    pub(crate) sg_id: SubgraphId,
    pub(crate) state_handle: Rc<Cell<Option<HandoffStateHandle<H>>>>,
    pub(crate) _phantom: PhantomData<fn() -> H>,
}
// impl<T: Clone> Clone for OutputPort<TeeingHandoff<T>> {
//     fn clone(&self) -> Self {
//         Self {
//             sg_id: self.sg_id,
//             handoff_id: Rc::new(RefCell::new(self.handoff.borrow().tee())),
//         }
//     }
// }

/**
 * Context provided to a compiled component for reading from an [InputPort].
 */
pub struct RecvCtx<'a, H>
where
    H: Handoff,
    H::State: 'static,
{
    pub(crate) state_handle: StateHandle<RefCell<H::State>>,
    pub(crate) context: &'a Context<'a>,
    pub(crate) _phantom: PhantomData<&'a mut H>,
}
impl<'a, H> Clone for RecvCtx<'a, H>
where
    H: Handoff,
    H::State: 'static,
{
    fn clone(&self) -> Self {
        Self {
            state_handle: self.state_handle,
            context: self.context,
            _phantom: PhantomData,
        }
    }
}
impl<'a, H> Copy for RecvCtx<'a, H>
where
    H: Handoff,
    H::State: 'static,
{
}

impl<'a, H> RecvCtx<'a, H>
where
    H: Handoff,
    H::State: 'static,
{
    pub fn take_inner(&self) -> H::Inner {
        let mut state = self.context.state_ref(self.state_handle).borrow_mut();
        H::take_inner(&mut *state)
    }
}

/**
 * Handle corresponding to a [RecvCtx]. Consumed by [crate::scheduled::Hydroflow::add_edge] to construct the Hydroflow graph.
 */
#[must_use]
pub struct InputPort<H: Handoff> {
    pub(crate) sg_id: SubgraphId,
    pub(crate) state_handle: Rc<Cell<Option<HandoffStateHandle<H>>>>,
    pub(crate) _phantom: PhantomData<fn() -> H>,
}
