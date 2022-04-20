use super::PullBuild;

use std::marker::PhantomData;

use crate::scheduled::context::Context;
use crate::scheduled::handoff::Handoff;
use crate::scheduled::port::RecvPort;
use crate::{tl, tt};

pub struct HandoffPullBuild<Hof>
where
    Hof: Handoff,
{
    _phantom: PhantomData<fn(Hof)>,
}

impl<Hof> Default for HandoffPullBuild<Hof>
where
    Hof: Handoff,
{
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Hof> HandoffPullBuild<Hof>
where
    Hof: Handoff,
{
    pub fn new() -> Self {
        Default::default()
    }
}

impl<Hof> PullBuild for HandoffPullBuild<Hof>
where
    Hof: Handoff,
{
    type ItemOut = Hof::Inner;
    type Build<'slf, 'ctx> = std::iter::Once<Hof::Inner>;

    type InputHandoffs = tt!(RecvPort<Hof>);

    fn build<'slf, 'ctx>(
        &'slf mut self,
        context: &'ctx mut Context,
        handoffs: &Self::InputHandoffs,
    ) -> Self::Build<'slf, 'ctx> {
        let tl!(handoff) = handoffs;
        std::iter::once(context.handoff_mut(handoff).take_inner())
    }
}
