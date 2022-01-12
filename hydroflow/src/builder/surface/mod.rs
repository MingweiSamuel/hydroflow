use super::build::{PullBuild, PushBuild};
use super::connect::{PullConnect, PushConnect};

pub mod pull_chain;
pub mod pull_join;

pub mod filter;
pub mod flat_map;
pub mod map;

use crate::scheduled::handoff::HandoffList;

pub trait BaseSurface {
    type ItemOut;
}

pub trait PullSurface: BaseSurface {
    type InputHandoffs: HandoffList;

    type Connect: PullConnect<InputHandoffs = Self::InputHandoffs>;
    type Build: PullBuild<InputHandoffs = Self::InputHandoffs, ItemOut = Self::ItemOut>;

    fn into_parts(self) -> (Self::Connect, Self::Build);
}

pub trait PushSurface: BaseSurface {
    type Output<Next>: PushSurfaceReversed
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>;

    fn reverse<Next>(self, next: Next) -> Self::Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>;
}

pub trait PushSurfaceReversed {
    type OutputHandoffs: HandoffList;

    type ItemIn;

    type Connect: PushConnect<OutputHandoffs = Self::OutputHandoffs>;
    type Build: PushBuild<OutputHandoffs = Self::OutputHandoffs, ItemIn = Self::ItemIn>;

    fn into_parts(self) -> (Self::Connect, Self::Build);
}
