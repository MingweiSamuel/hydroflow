use super::{Pull, PullBase};

use std::marker::PhantomData;

use crate::scheduled::handoff::{Handoff, HandoffList};
use crate::{tl, tt};

pub struct HandoffPull<H>(PhantomData<*const H>);
impl<H: Handoff> HandoffPull<H> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}
impl<H: Handoff> PullBase for HandoffPull<H> {
    type Item = H::Inner;
    type Build<'i> = std::array::IntoIter<Self::Item, 1>;
}
impl<H: Handoff> Pull for HandoffPull<H> {
    type InputHandoffs = tt!(H);

    fn init(&mut self, input_ports: <Self::InputHandoffs as HandoffList>::InputPort) {
        todo!();
    }

    fn build(
        &mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'_>,
    ) -> Self::Build<'_> {
        let tl!(handoff) = input;
        [handoff.take_inner()].into_iter()
    }
}
