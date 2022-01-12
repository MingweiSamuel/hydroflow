use super::build::{PullBuild, PushBuild};
use super::connect::{PullConnect, PushConnect};

pub mod pull_chain;
pub mod pull_filter;
pub mod pull_flat_map;
pub mod pull_map;

use crate::scheduled::handoff::HandoffList;

pub trait PullSurface {
    type InputHandoffs: HandoffList;

    type ItemOut;

    type Connect: PullConnect<InputHandoffs = Self::InputHandoffs>;
    type Build: PullBuild<InputHandoffs = Self::InputHandoffs, ItemOut = Self::ItemOut>;

    fn into_parts(self) -> (Self::Connect, Self::Build);
}

pub trait PushSurface {
    type ItemOut;

    type Output<Next>
    // TODO(mingwei): trait bound.
    where
        Next: PushSurface;

    fn reverse<Next>(self, next: Next) -> Self::Output<Next>
    where
        Next: PushSurface;
}

pub trait PushSurfaceReversed {
    type ItemIn;

    type Connect: PushConnect;
    type Build: PushBuild<ItemIn = Self::ItemIn>;
}
