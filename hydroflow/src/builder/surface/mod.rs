use super::build::{PullBuild, PushBuild};
use super::connect::{PullConnect, PushConnect};

pub mod filter;
pub mod flat_map;
pub mod map;

pub mod pull_chain;
pub mod pull_handoff;
pub mod pull_join;

pub mod push_handoff;
pub mod push_tee;

use std::hash::Hash;

use crate::scheduled::handoff::{HandoffList, HandoffListSplit};
use crate::scheduled::type_list::Extend;

/// Common trait shared between push and pull surface APIs.
///
/// Provides non-push/pull-specific chaining methods.
pub trait BaseSurface {
    type ItemOut;

    fn map<Func, Out>(self, func: Func) -> map::MapSurface<Self, Func>
    where
        Self: Sized,
        Func: FnMut(Self::ItemOut) -> Out,
    {
        map::MapSurface::new(self, func)
    }

    fn flat_map<Func, Out>(self, func: Func) -> flat_map::FlatMapSurface<Self, Func>
    where
        Self: Sized,
        Func: FnMut(Self::ItemOut) -> Out,
        Out: IntoIterator,
    {
        flat_map::FlatMapSurface::new(self, func)
    }

    fn filter<Func, Out>(self, func: Func) -> filter::FilterSurface<Self, Func>
    where
        Self: Sized,
        Func: FnMut(&Self::ItemOut) -> bool,
    {
        filter::FilterSurface::new(self, func)
    }
}

pub trait PullSurface: BaseSurface {
    type InputHandoffs: HandoffList;

    type Connect: PullConnect<InputHandoffs = Self::InputHandoffs>;
    type Build: PullBuild<InputHandoffs = Self::InputHandoffs, ItemOut = Self::ItemOut>;

    fn into_parts(self) -> (Self::Connect, Self::Build);

    fn chain<Other>(self, other: Other) -> pull_chain::ChainPullSurface<Self, Other>
    where
        Self: Sized,
        Other: PullSurface<ItemOut = Self::ItemOut>,
    {
        pull_chain::ChainPullSurface::new(self, other)
    }

    fn join<Other, Key, ValSelf, ValOther>(
        self,
        other: Other,
    ) -> pull_join::JoinPullSurface<Self, Other>
    where
        Self: Sized + PullSurface<ItemOut = (Key, ValSelf)>,
        Other: PullSurface<ItemOut = (Key, ValOther)>,
        Key: 'static + Eq + Hash + Clone,
        ValSelf: 'static + Eq + Clone,
        ValOther: 'static + Eq + Clone,
    {
        pull_join::JoinPullSurface::new(self, other)
    }
}

pub trait PushSurface: BaseSurface {
    type Output<Next>: PushSurfaceReversed
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>;

    fn reverse<Next>(self, next: Next) -> Self::Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>;

    fn tee<NextA, NextB>(
        self,
        next_a: NextA,
        next_b: NextB,
    ) -> Self::Output<push_tee::TeePushSurfaceReversed<NextA, NextB>>
    where
        Self: Sized,
        Self::ItemOut: Clone,
        NextA: PushSurfaceReversed<ItemIn = Self::ItemOut>,
        NextB: PushSurfaceReversed<ItemIn = Self::ItemOut>,

        NextA::OutputHandoffs: Extend<NextB::OutputHandoffs>,
        <NextA::OutputHandoffs as Extend<NextB::OutputHandoffs>>::Extended:
            HandoffList + HandoffListSplit<NextA::OutputHandoffs, Suffix = NextB::OutputHandoffs>,
    {
        let next = push_tee::TeePushSurfaceReversed::new(next_a, next_b);
        self.reverse(next)
    }
}

pub trait PushSurfaceReversed {
    type OutputHandoffs: HandoffList;

    type ItemIn;

    type Connect: PushConnect<OutputHandoffs = Self::OutputHandoffs>;
    type Build: PushBuild<OutputHandoffs = Self::OutputHandoffs, ItemIn = Self::ItemIn>;

    fn into_parts(self) -> (Self::Connect, Self::Build);
}
