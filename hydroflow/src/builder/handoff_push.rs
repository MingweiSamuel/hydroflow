use super::{Push, PushBase};

use std::marker::PhantomData;

use crate::compiled::push_handoff::PushHandoff;
use crate::scheduled::handoff::{CanReceive, Handoff, HandoffList};
use crate::{tl, tt};

pub struct HandoffPush<H, T>(PhantomData<*const H>, PhantomData<fn(T)>)
where
    H: Handoff + CanReceive<T>;
impl<H, T> HandoffPush<H, T>
where
    H: Handoff + CanReceive<T>,
{
    pub(crate) fn new() -> Self {
        Self(PhantomData, PhantomData)
    }
}

impl<H, T> PushBase for HandoffPush<H, T>
where
    H: Handoff + CanReceive<T>,
{
    type Item = T;
    type Build<'a, 'i> = PushHandoff<'i, H, T>;
}
impl<H, T> Push for HandoffPush<H, T>
where
    H: Handoff + CanReceive<T>,
{
    type OutputHandoffs = tt!(H);

    fn init(&mut self, output_ports: <Self::OutputHandoffs as HandoffList>::OutputPort) {
        todo!()
    }

    fn build<'a, 'i>(
        &'a mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'i>,
    ) -> Self::Build<'a, 'i> {
        let tl!(handoff) = input;
        PushHandoff::new(handoff)
    }
}
