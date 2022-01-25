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
pub struct FlattenSurface<Prev>
where
    Prev: BaseSurface,
{
    prev: Prev,
}
impl<Prev> FlattenSurface<Prev>
where
    Prev: BaseSurface,
    Prev::ItemOut: IntoIterator,
{
    pub fn new(prev: Prev) -> Self {
        Self { prev }
    }
}
impl<Prev> BaseSurface for FlattenSurface<Prev>
where
    Prev: BaseSurface,
    Prev::ItemOut: IntoIterator,
{
    type ItemOut = <Prev::ItemOut as IntoIterator>::Item;
}
impl<Prev> PullSurface for FlattenSurface<Prev>
where
    Prev: PullSurface,
    Prev::ItemOut: IntoIterator,
{
    type InputHandoffs = Prev::InputHandoffs;
    type Connect = Prev::Connect;
    type Build = FlattenPullBuild<Prev::Build>;
    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let Self { prev } = self;
        let (connect, build) = prev.into_parts();
        let build = FlattenPullBuild::new(build);
        (connect, build)
    }
}
impl<Prev> PushSurface for FlattenSurface<Prev>
where
    Prev: PushSurface,
    Prev::ItemOut: IntoIterator,
{
    type Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
    = Prev::Output<FlattenPushSurfaceReversed<Next, Prev::ItemOut>>;
    fn reverse<Next>(self, next: Next) -> Self::Output<Next>
    where
        Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
    {
        let Self { prev } = self;
        prev.reverse(FlattenPushSurfaceReversed::new(next))
    }
}
pub struct FlattenPullBuild<Prev>
where
    Prev: PullBuild,
{
    prev: Prev,
}
impl<Prev> FlattenPullBuild<Prev>
where
    Prev: PullBuild,
    Prev::ItemOut: IntoIterator,
{
    pub fn new(prev: Prev) -> Self {
        Self { prev }
    }
}
#[allow(type_alias_bounds)]
type FlattenPullBuildOutput<'slf, 'hof, Prev>
where
    Prev: PullBuild,
    Prev::ItemOut: IntoIterator,
= impl Iterator<Item = <Prev::ItemOut as IntoIterator>::Item>;
impl<Prev> PullBuildBase for FlattenPullBuild<Prev>
where
    Prev: PullBuild,
    Prev::ItemOut: IntoIterator,
{
    type ItemOut = <Prev::ItemOut as IntoIterator>::Item;
    type Build<'slf, 'hof> = FlattenPullBuildOutput<'slf, 'hof, Prev>;
}
impl<Prev> PullBuild for FlattenPullBuild<Prev>
where
    Prev: PullBuild,
    Prev::ItemOut: IntoIterator,
{
    type InputHandoffs = Prev::InputHandoffs;
    fn build<'slf, 'hof>(
        &'slf mut self,
        handoffs: <Self::InputHandoffs as HandoffList>::RecvCtx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        let Self { prev } = self;
        prev.build(handoffs).flatten()
    }
}
pub struct FlattenPushSurfaceReversed<Next, ItemIn>
where
    Next: PushSurfaceReversed<ItemIn = <ItemIn as IntoIterator>::Item>,
    ItemIn: IntoIterator,
{
    next: Next,
    _phantom: std::marker::PhantomData<fn(ItemIn)>,
}
impl<Next, ItemIn> FlattenPushSurfaceReversed<Next, ItemIn>
where
    Next: PushSurfaceReversed<ItemIn = <ItemIn as IntoIterator>::Item>,
    ItemIn: IntoIterator,
{
    pub fn new(next: Next) -> Self {
        Self {
            next,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<Next, ItemIn> PushSurfaceReversed for FlattenPushSurfaceReversed<Next, ItemIn>
where
    Next: PushSurfaceReversed<ItemIn = <ItemIn as IntoIterator>::Item>,
    ItemIn: IntoIterator,
{
    type OutputHandoffs = Next::OutputHandoffs;
    type ItemIn = ItemIn;
    type Connect = Next::Connect;
    type Build = FlattenPushBuild<Next::Build, ItemIn>;
    fn into_parts(self) -> (Self::Connect, Self::Build) {
        let Self { next, _phantom } = self;
        let (connect, build) = next.into_parts();
        let build = FlattenPushBuild::new(build);
        (connect, build)
    }
}
pub struct FlattenPushBuild<Next, ItemIn>
where
    Next: PushBuild<ItemIn = <ItemIn as IntoIterator>::Item>,
    ItemIn: IntoIterator,
{
    next: Next,
    _phantom: std::marker::PhantomData<fn(ItemIn)>,
}
impl<Next, ItemIn> FlattenPushBuild<Next, ItemIn>
where
    Next: PushBuild<ItemIn = <ItemIn as IntoIterator>::Item>,
    ItemIn: IntoIterator,
{
    pub fn new(next: Next) -> Self {
        Self {
            next,
            _phantom: std::marker::PhantomData,
        }
    }
}
#[allow(type_alias_bounds)]
type FlattenPushBuildOutput<'slf, 'hof, Next, ItemIn>
where
    Next: PushBuild<ItemIn = <ItemIn as IntoIterator>::Item>,
    ItemIn: IntoIterator,
= impl Pusherator<Item = ItemIn>;
impl<Next, ItemIn> PushBuildBase for FlattenPushBuild<Next, ItemIn>
where
    Next: PushBuild<ItemIn = <ItemIn as IntoIterator>::Item>,
    ItemIn: IntoIterator,
{
    type ItemIn = ItemIn;
    type Build<'slf, 'hof> = FlattenPushBuildOutput<'slf, 'hof, Next, ItemIn>;
}
impl<Next, ItemIn> PushBuild for FlattenPushBuild<Next, ItemIn>
where
    Next: PushBuild<ItemIn = <ItemIn as IntoIterator>::Item>,
    ItemIn: IntoIterator,
{
    type OutputHandoffs = Next::OutputHandoffs;
    fn build<'slf, 'hof>(
        &'slf mut self,
        handoffs: <Self::OutputHandoffs as HandoffList>::SendCtx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        let Self { next, _phantom } = self;
        hydroflow::compiled::flatten::Flatten::new(next.build(handoffs))
    }
}
