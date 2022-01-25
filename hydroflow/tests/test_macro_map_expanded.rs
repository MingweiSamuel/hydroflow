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
pub struct MapSurface<Prev, F>
where
    Prev: BaseSurface,
{
    prev: Prev,
    f: F,
}
impl<Prev, B, F> MapSurface<Prev, F>
where
    Prev: BaseSurface,
    F: FnMut(Prev::ItemOut) -> B,
{
    pub fn new(prev: Prev, f: F) -> Self {
        Self { prev, f }
    }
}
impl<Prev, B, F> BaseSurface for MapSurface<Prev, F>
where
    Prev: BaseSurface,
    F: FnMut(Prev::ItemOut) -> B,
{
    type ItemOut = B;
}
impl<Prev, B, F> PullSurface for MapSurface<Prev, F>
where
    Prev: PullSurface,
    F: FnMut(Prev::ItemOut) -> B,
{
    type InputHandoffs = Prev::InputHandoffs;
    type Connect = Prev::Connect;
    type Build = MapPullBuild<Prev::Build, F>;
    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let Self { prev, f } = self;
        let (connect, build) = prev.into_parts();
        let build = MapPullBuild::new(build, f);
        (connect, build)
    }
}
impl<Prev, B, F> PushSurface for MapSurface<Prev, F>
where
    Prev: PushSurface,
    F: FnMut(Prev::ItemOut) -> B,
{
    type Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
    = Prev::Output<MapPushSurfaceReversed<Next, F, Prev::ItemOut>>;
    fn reverse<Next>(self, next: Next) -> Self::Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
    {
        let Self { prev, f } = self;
        prev.reverse(MapPushSurfaceReversed::new(next, f))
    }
}
pub struct MapPullBuild<Prev, F>
where
    Prev: PullBuild,
{
    prev: Prev,
    f: F,
}
impl<Prev, B, F> MapPullBuild<Prev, F>
where
    Prev: PullBuild,
    F: FnMut(Prev::ItemOut) -> B,
{
    pub fn new(prev: Prev, f: F) -> Self {
        Self { prev, f }
    }
}
#[allow(type_alias_bounds)]
type MapPullBuildOutput<'slf, 'hof, Prev, B, F>
where
    Prev: PullBuild,
    F: FnMut(Prev::ItemOut) -> B,
= impl Iterator<Item = B>;
impl<Prev, B, F> PullBuildBase for MapPullBuild<Prev, F>
where
    Prev: PullBuild,
    F: FnMut(Prev::ItemOut) -> B,
{
    type ItemOut = B;
    type Build<'slf, 'hof> = MapPullBuildOutput<'slf, 'hof, Prev, B, F>;
}
impl<Prev, B, F> PullBuild for MapPullBuild<Prev, F>
where
    Prev: PullBuild,
    F: FnMut(Prev::ItemOut) -> B,
{
    type InputHandoffs = Prev::InputHandoffs;
    fn build<'slf, 'hof>(
        &'slf mut self,
        handoffs: <Self::InputHandoffs as HandoffList>::RecvCtx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        let Self { prev, f } = self;
        prev.build(handoffs).map(|x| (f)(x))
    }
}
pub struct MapPushSurfaceReversed<Next, F, ItemIn>
where
    Next: PushSurfaceReversed,
    F: FnMut(ItemIn) -> Next::ItemIn,
{
    next: Next,
    f: F,
    _phantom: std::marker::PhantomData<fn(ItemIn)>,
}
impl<Next, F, ItemIn> MapPushSurfaceReversed<Next, F, ItemIn>
where
    Next: PushSurfaceReversed,
    F: FnMut(ItemIn) -> Next::ItemIn,
{
    pub fn new(next: Next, f: F) -> Self {
        Self {
            next,
            f,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<Next, F, ItemIn> PushSurfaceReversed for MapPushSurfaceReversed<Next, F, ItemIn>
where
    Next: PushSurfaceReversed,
    F: FnMut(ItemIn) -> Next::ItemIn,
{
    type OutputHandoffs = Next::OutputHandoffs;
    type ItemIn = ItemIn;
    type Connect = Next::Connect;
    type Build = MapPushBuild<Next::Build, F, ItemIn>;
    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let Self { next, f, _phantom } = self;
        let (connect, build) = next.into_parts();
        let build = MapPushBuild::new(build, f);
        (connect, build)
    }
}
pub struct MapPushBuild<Next, F, ItemIn>
where
    Next: PushBuild,
    F: FnMut(ItemIn) -> Next::ItemIn,
{
    next: Next,
    f: F,
    _phantom: std::marker::PhantomData<fn(ItemIn)>,
}
impl<Next, F, ItemIn> MapPushBuild<Next, F, ItemIn>
where
    Next: PushBuild,
    F: FnMut(ItemIn) -> Next::ItemIn,
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
type MapPushBuildOutput<'slf, 'hof, Next, F, ItemIn>
where
    Next: PushBuild,
    F: FnMut(ItemIn) -> Next::ItemIn,
= impl Pusherator<Item = ItemIn>;
impl<Next, F, ItemIn> PushBuildBase for MapPushBuild<Next, F, ItemIn>
where
    Next: PushBuild,
    F: FnMut(ItemIn) -> Next::ItemIn,
{
    type ItemIn = ItemIn;
    type Build<'slf, 'hof> = MapPushBuildOutput<'slf, 'hof, Next, F, ItemIn>;
}
impl<Next, F, ItemIn> PushBuild for MapPushBuild<Next, F, ItemIn>
where
    Next: PushBuild,
    F: FnMut(ItemIn) -> Next::ItemIn,
{
    type OutputHandoffs = Next::OutputHandoffs;
    fn build<'slf, 'hof>(
        &'slf mut self,
        handoffs: <Self::OutputHandoffs as HandoffList>::SendCtx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        let Self { next, f, _phantom } = self;
        hydroflow::compiled::map::Map::new(|x| (f)(x), next.build(handoffs))
    }
}
