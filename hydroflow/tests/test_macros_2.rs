#![feature(prelude_import)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use hydroflow::builder::build::{PullBuild, PullBuildBase, PushBuild, PushBuildBase};
use hydroflow::builder::surface::{BaseSurface, PullSurface, PushSurface, PushSurfaceReversed};
use hydroflow::scheduled::handoff::HandoffList;
use hydroflow::compiled::Pusherator;
pub struct FilterMapSurface<Prev, F>
where
    Prev: BaseSurface,
{
    prev: Prev,
    f: F,
}
impl<Prev, B, F> FilterMapSurface<Prev, F>
where
    Prev: BaseSurface,
    F: FnMut(Prev::ItemOut) -> Option<B>,
{
    pub fn new(prev: Prev, f: F) -> Self {
        Self { prev, f }
    }
}
impl<Prev, B, F> BaseSurface for FilterMapSurface<Prev, F>
where
    Prev: BaseSurface,
    F: FnMut(Prev::ItemOut) -> Option<B>,
{
    type ItemOut = B;
}
impl<Prev, B, F> PullSurface for FilterMapSurface<Prev, F>
where
    Prev: PullSurface,
    F: FnMut(Prev::ItemOut) -> Option<B>,
{
    type InputHandoffs = Prev::InputHandoffs;
    type Connect = Prev::Connect;
    type Build = FilterMapPullBuild<Prev::Build, F>;
    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let Self { prev, f } = self;
        let (connect, build) = prev.into_parts();
        let build = FilterMapPullBuild::new(build, f);
        (connect, build)
    }
}
impl<Prev, B, F> PushSurface for FilterMapSurface<Prev, F>
where
    Prev: PushSurface,
    F: FnMut(Prev::ItemOut) -> Option<B>,
{
    type Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
    = Prev::Output<FilterMapPushSurfaceReversed<Next, F, Prev::ItemOut>>;
    fn reverse<Next>(self, next: Next) -> Self::Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
    {
        let Self { prev, f } = self;
        prev.reverse(FilterMapPushSurfaceReversed::new(next, f))
    }
}
pub struct FilterMapPushSurfaceReversed<Next, F, ItemIn>
where
    Next: PushSurfaceReversed,
    F: FnMut(ItemIn) -> Option<Next::ItemIn>,
{
    next: Next,
    f: F,
    _phantom: std::marker::PhantomData<fn(ItemIn)>,
}
impl<Next, F, ItemIn> FilterMapPushSurfaceReversed<Next, F, ItemIn>
where
    Next: PushSurfaceReversed,
    F: FnMut(ItemIn) -> Option<Next::ItemIn>,
{
    pub fn new(next: Next, f: F) -> Self {
        Self {
            next,
            f,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<Next, F, ItemIn> PushSurfaceReversed for FilterMapPushSurfaceReversed<Next, F, ItemIn>
where
    Next: PushSurfaceReversed,
    F: FnMut(ItemIn) -> Option<Next::ItemIn>,
{
    type OutputHandoffs = Next::OutputHandoffs;
    type ItemIn = ItemIn;
    type Connect = Next::Connect;
    type Build = FilterMapPushBuild<Next::Build, F, ItemIn>;
    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let Self { next, f, _phantom } = self;
        let (connect, build) = next.into_parts();
        let build = FilterMapPushBuild::new(build, f);
        (connect, build)
    }
}
pub struct FilterMapPushBuild<Next, F, ItemIn>
where
    Next: PushBuild,
    F: FnMut(ItemIn) -> Option<Next::ItemIn>,
{
    next: Next,
    f: F,
    _phantom: std::marker::PhantomData<fn(ItemIn)>,
}
impl<Next, F, ItemIn> FilterMapPushBuild<Next, F, ItemIn>
where
    Next: PushBuild,
    F: FnMut(ItemIn) -> Option<Next::ItemIn>,
{
    pub fn new(next: Next, f: F) -> Self {
        Self {
            next,
            f,
            _phantom: std::marker::PhantomData,
        }
    }
}
#[allow(type_alias_bounds)]
type FilterMapPushBuildImpl<'slf, 'hof, Next, F, ItemIn>
where
    Next: PushBuild,
= impl Pusherator<Item = ItemIn>;
impl<Next, F, ItemIn> PushBuildBase for FilterMapPushBuild<Next, F, ItemIn>
where
    Next: PushBuild,
    F: FnMut(ItemIn) -> Option<Next::ItemIn>,
{
    type ItemIn = ItemIn;
    type Build<'slf, 'hof> = FilterMapPushBuildImpl<'slf, 'hof, Next, F, ItemIn>;
}
impl<Next, F, ItemIn> PushBuild for FilterMapPushBuild<Next, F, ItemIn>
where
    Next: PushBuild,
    F: FnMut(ItemIn) -> Option<Next::ItemIn>,
{
    type OutputHandoffs = Next::OutputHandoffs;
    fn build<'slf, 'hof>(
        &'slf mut self,
        handoffs: <Self::OutputHandoffs as HandoffList>::SendCtx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        hydroflow::compiled::filter_map::FilterMap::new(
            |x| (self.f)(x),
            self.next.build(handoffs),
        )
    }
}
pub struct FilterMapPullBuild<Prev, F>
where
    Prev: PullBuild,
{
    prev: Prev,
    f: F,
}
impl<Prev, B, F> FilterMapPullBuild<Prev, F>
where
    Prev: PullBuild,
    F: FnMut(Prev::ItemOut) -> Option<B>,
{
    pub fn new(prev: Prev, f: F) -> Self {
        Self { prev, f }
    }
}