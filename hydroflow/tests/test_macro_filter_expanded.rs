#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use hydroflow::builder::build::{PullBuild, PullBuildBase, PushBuild, PushBuildBase};
use hydroflow::builder::surface::{BaseSurface, PullSurface, PushSurface, PushSurfaceReversed};
use hydroflow::scheduled::handoff::HandoffList;
use hydroflow::compiled::Pusherator;


pub struct FilterSurface<Prev, P>
where
    Prev: BaseSurface,
{
    prev: Prev,
    predicate: P,
}
impl<Prev, P> FilterSurface<Prev, P>
where
    Prev: BaseSurface,
    P: FnMut(&Prev::ItemOut) -> bool,
{
    pub fn new(prev: Prev, predicate: P) -> Self {
        Self { prev, predicate }
    }
}
impl<Prev, P> BaseSurface for FilterSurface<Prev, P>
where
    Prev: BaseSurface,
    P: FnMut(&Prev::ItemOut) -> bool,
{
    type ItemOut = Prev::ItemOut;
}
impl<Prev, P> PullSurface for FilterSurface<Prev, P>
where
    Prev: PullSurface,
    P: FnMut(&Prev::ItemOut) -> bool,
{
    type InputHandoffs = Prev::InputHandoffs;
    type Connect = Prev::Connect;
    type Build = FilterPullBuild<Prev::Build, P>;
    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let Self { prev, predicate } = self;
        let (connect, build) = prev.into_parts();
        let build = FilterPullBuild::new(build, predicate);
        (connect, build)
    }
}
impl<Prev, P> PushSurface for FilterSurface<Prev, P>
where
    Prev: PushSurface,
    P: FnMut(&Prev::ItemOut) -> bool,
{
    type Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
    = Prev::Output<FilterPushSurfaceReversed<Next, P>>;
    fn reverse<Next>(self, next: Next) -> Self::Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
    {
        let Self { prev, predicate } = self;
        prev.reverse(FilterPushSurfaceReversed::new(next, predicate))
    }
}
pub struct FilterPullBuild<Prev, P>
where
    Prev: PullBuild,
{
    prev: Prev,
    predicate: P,
}
impl<Prev, P> FilterPullBuild<Prev, P>
where
    Prev: PullBuild,
    P: FnMut(&Prev::ItemOut) -> bool,
{
    pub fn new(prev: Prev, predicate: P) -> Self {
        Self { prev, predicate }
    }
}
#[allow(type_alias_bounds)]
type FilterPullBuildOutput<'slf, 'hof, Prev, P>
where
    Prev: PullBuild,
    P: FnMut(&Prev::ItemOut) -> bool,
= impl Iterator<Item = Prev::ItemOut>;
impl<Prev, P> PullBuildBase for FilterPullBuild<Prev, P>
where
    Prev: PullBuild,
    P: FnMut(&Prev::ItemOut) -> bool,
{
    type ItemOut = Prev::ItemOut;
    type Build<'slf, 'hof> = FilterPullBuildOutput<'slf, 'hof, Prev, P>;
}
impl<Prev, P> PullBuild for FilterPullBuild<Prev, P>
where
    Prev: PullBuild,
    P: FnMut(&Prev::ItemOut) -> bool,
{
    type InputHandoffs = Prev::InputHandoffs;
    fn build<'slf, 'hof>(
        &'slf mut self,
        handoffs: <Self::InputHandoffs as HandoffList>::RecvCtx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        let Self { prev, predicate } = self;
        prev.build(handoffs).filter(predicate)
    }
}
pub struct FilterPushSurfaceReversed<Next, P>
where
    Next: PushSurfaceReversed,
    P: FnMut(&Next::ItemIn) -> bool,
{
    next: Next,
    predicate: P,
    _phantom: std::marker::PhantomData<fn()>,
}
impl<Next, P> FilterPushSurfaceReversed<Next, P>
where
    Next: PushSurfaceReversed,
    P: FnMut(&Next::ItemIn) -> bool,
{
    pub fn new(next: Next, predicate: P) -> Self {
        Self {
            next,
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<Next, P> PushSurfaceReversed for FilterPushSurfaceReversed<Next, P>
where
    Next: PushSurfaceReversed,
    P: FnMut(&Next::ItemIn) -> bool,
{
    type OutputHandoffs = Next::OutputHandoffs;
    type ItemIn = Next::ItemIn;
    type Connect = Next::Connect;
    type Build = FilterPushBuild<Next::Build, P>;
    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let Self {
            next,
            predicate,
            _phantom,
        } = self;
        let (connect, build) = next.into_parts();
        let build = FilterPushBuild::new(build, predicate);
        (connect, build)
    }
}
pub struct FilterPushBuild<Next, P>
where
    Next: PushBuild,
    P: FnMut(&Next::ItemIn) -> bool,
{
    next: Next,
    predicate: P,
    _phantom: std::marker::PhantomData<fn()>,
}
impl<Next, P> FilterPushBuild<Next, P>
where
    Next: PushBuild,
    P: FnMut(&Next::ItemIn) -> bool,
{
    pub fn new(next: Next, predicate: P) -> Self {
        Self {
            next,
            predicate,
            _phantom: std::marker::PhantomData,
        }
    }
}
#[allow(type_alias_bounds)]
type FilterPushBuildOutput<'slf, 'hof, Next, P>
where
    Next: PushBuild,
    P: FnMut(&Next::ItemIn) -> bool,
= impl Pusherator<Item = Next::ItemIn>;
impl<Next, P> PushBuildBase for FilterPushBuild<Next, P>
where
    Next: PushBuild,
    P: FnMut(&Next::ItemIn) -> bool,
{
    type ItemIn = Next::ItemIn;
    type Build<'slf, 'hof> = FilterPushBuildOutput<'slf, 'hof, Next, P>;
}
impl<Next, P> PushBuild for FilterPushBuild<Next, P>
where
    Next: PushBuild,
    P: FnMut(&Next::ItemIn) -> bool,
{
    type OutputHandoffs = Next::OutputHandoffs;
    fn build<'slf, 'hof>(
        &'slf mut self,
        handoffs: <Self::OutputHandoffs as HandoffList>::SendCtx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        let Self {
            next,
            predicate,
            _phantom,
        } = self;
        hydroflow::compiled::filter::Filter::new(predicate, next.build(handoffs))
    }
}
