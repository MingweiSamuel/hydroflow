use super::{BaseSurface, PullSurface, PushSurface, PushSurfaceReversed};

use std::marker::PhantomData;

use crate::builder::build::pull_filter_map::FilterMapPullBuild;
use crate::builder::build::push_filter_map::FilterMapPushBuild;

pub struct FilterMapSurface<Prev, Func> //FUNC
where
    Prev: BaseSurface,
{
    prev: Prev,
    func: Func, //FUNC
}
impl<Prev, Func, Out> FilterMapSurface<Prev, Func>
where
    Prev: BaseSurface,
    Func: FnMut(Prev::ItemOut) -> Option<Out>, // EXTRA BOUND
{
    pub fn new(prev: Prev, func: Func) -> Self {
        Self { prev, func }
    }
}

impl<Prev, Func, Out> BaseSurface for FilterMapSurface<Prev, Func>
where
    Prev: BaseSurface,
    Func: FnMut(Prev::ItemOut) -> Option<Out>,
{
    type ItemOut = Out;
}

impl<Prev, Func, Out> PullSurface for FilterMapSurface<Prev, Func>
where
    Prev: PullSurface,
    Func: FnMut(Prev::ItemOut) -> Option<Out>,
{
    type InputHandoffs = Prev::InputHandoffs; // HERE?

    type Connect = Prev::Connect;
    type Build = FilterMapPullBuild<Prev::Build, Func>;

    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let (connect, build) = self.prev.into_parts();
        let build = FilterMapPullBuild::new(build, self.func);
        (connect, build)
    }
}

impl<Prev, Func, Out> PushSurface for FilterMapSurface<Prev, Func>
where
    Prev: PushSurface,
    Func: FnMut(Prev::ItemOut) -> Option<Out>,
{
    type Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
    = Prev::Output<FilterMapPushSurfaceReversed<Next, Func, Prev::ItemOut>>;

    fn reverse<Next>(self, next: Next) -> Self::Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
    {
        self.prev
            .reverse(FilterMapPushSurfaceReversed::new(next, self.func))
    }
}

pub struct FilterMapPushSurfaceReversed<Next, Func, In>
where
    Next: PushSurfaceReversed,
{
    next: Next,
    func: Func,
    _phantom: PhantomData<fn(In)>,
}
impl<Next, Func, In> FilterMapPushSurfaceReversed<Next, Func, In>
where
    Next: PushSurfaceReversed,
    Func: FnMut(In) -> Option<Next::ItemIn>,
{
    pub fn new(next: Next, func: Func) -> Self {
        Self {
            next,
            func,
            _phantom: PhantomData,
        }
    }
}

impl<Next, Func, In> PushSurfaceReversed for FilterMapPushSurfaceReversed<Next, Func, In>
where
    Next: PushSurfaceReversed,
    Func: FnMut(In) -> Option<Next::ItemIn>,
{
    type OutputHandoffs = Next::OutputHandoffs;

    type ItemIn = In;

    type Connect = Next::Connect;
    type Build = FilterMapPushBuild<Next::Build, Func, In>;

    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let (connect, build) = self.next.into_parts();
        let build = FilterMapPushBuild::new(build, self.func);
        (connect, build)
    }
}

use super::{PullBuild, PullBuildBase};

use crate::scheduled::handoff::HandoffList;

pub struct FilterMapPullBuild<Prev, Func>
where
    Prev: PullBuild,
{
    prev: Prev,
    func: Func,
}
impl<Prev, Func, Out> FilterMapPullBuild<Prev, Func>
where
    Prev: PullBuild,
    Func: FnMut(Prev::ItemOut) -> Option<Out>,
{
    pub fn new(prev: Prev, func: Func) -> Self {
        Self { prev, func }
    }
}

#[allow(type_alias_bounds)]
type PullBuildImpl<'slf, 'hof, Prev, Func, Out>
where
    Prev: PullBuild,
= std::iter::FilterMap<Prev::Build<'slf, 'hof>, impl FnMut(Prev::ItemOut) -> Option<Out>>;

impl<Prev, Func, Out> PullBuildBase for FilterMapPullBuild<Prev, Func>
where
    Prev: PullBuild,
    Func: FnMut(Prev::ItemOut) -> Option<Out>,
{
    type ItemOut = Out;
    type Build<'slf, 'hof> = PullBuildImpl<'slf, 'hof, Prev, Func, Out>;
}

impl<Prev, Func, Out> PullBuild for FilterMapPullBuild<Prev, Func>
where
    Prev: PullBuild,
    Func: FnMut(Prev::ItemOut) -> Option<Out>,
{
    type InputHandoffs = Prev::InputHandoffs;

    fn build<'slf, 'hof>(
        &'slf mut self,
        handoffs: <Self::InputHandoffs as HandoffList>::RecvCtx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        self.prev.build(handoffs).filter_map(|x| (self.func)(x))
    }
}

use super::{PushBuild, PushBuildBase};

use std::marker::PhantomData;

use crate::compiled::filter_map::FilterMap;
use crate::scheduled::handoff::HandoffList;

pub struct FilterMapPushBuild<Next, Func, In>
where
    Next: PushBuild,
    Func: FnMut(In) -> Option<Next::ItemIn>,
{
    next: Next,
    func: Func,
    _phantom: PhantomData<fn(In)>,
}
impl<Next, Func, In> FilterMapPushBuild<Next, Func, In>
where
    Next: PushBuild,
    Func: FnMut(In) -> Option<Next::ItemIn>,
{
    pub fn new(next: Next, func: Func) -> Self {
        Self {
            next,
            func,
            _phantom: PhantomData,
        }
    }
}

#[allow(type_alias_bounds)]
type PushBuildImpl<'slf, 'hof, Next, Func, In>
where
    Next: PushBuild,
= FilterMap<Next::Build<'slf, 'hof>, impl FnMut(In) -> Option<Next::ItemIn>, In>;

impl<Next, Func, In> PushBuildBase for FilterMapPushBuild<Next, Func, In>
where
    Next: PushBuild,
    Func: FnMut(In) -> Option<Next::ItemIn>,
{
    type ItemIn = In;
    type Build<'slf, 'hof> = PushBuildImpl<'slf, 'hof, Next, Func, In>;
}

impl<Next, Func, In> PushBuild for FilterMapPushBuild<Next, Func, In>
where
    Next: PushBuild,
    Func: FnMut(In) -> Option<Next::ItemIn>,
{
    type OutputHandoffs = Next::OutputHandoffs;

    fn build<'slf, 'hof>(
        &'slf mut self,
        handoffs: <Self::OutputHandoffs as HandoffList>::SendCtx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        FilterMap::new(|x| (self.func)(x), self.next.build(handoffs))
    }
}
