use super::{BuildBase, PullBuild, PullBuildBase, PushBuild, PushBuildBase};

use crate::compiled::flat_map::FlatMap;
use crate::compiled::Pusherator;
use crate::scheduled::handoff::HandoffList;

pub struct FlatMapBuild<I, F>
where
    I: BuildBase,
{
    prev: I,
    func: F,
}
impl<I, F, U> FlatMapBuild<I, F>
where
    I: BuildBase,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
    pub fn new(prev: I, func: F) -> Self {
        Self { prev, func }
    }
}

impl<I, F, U> BuildBase for FlatMapBuild<I, F>
where
    I: BuildBase,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
    type Item = U::Item;
}

#[allow(type_alias_bounds)]
type PullBuildImpl<'slf, 'inp, I, F, U>
where
    I: PullBuild,
= std::iter::FlatMap<I::Build<'slf, 'inp>, U, impl 'slf + FnMut(I::Item) -> U>;

impl<I, F, U> PullBuildBase for FlatMapBuild<I, F>
where
    I: PullBuild,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
    type Build<'slf, 'inp> = PullBuildImpl<'slf, 'inp, I, F, U>;
}
impl<I, F, U> PullBuild for FlatMapBuild<I, F>
where
    I: PullBuild,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
    type InputHandoffs = I::InputHandoffs;

    fn build<'slf, 'inp>(
        &'slf mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'inp>,
    ) -> Self::Build<'slf, 'inp> {
        self.prev.build(input).flat_map(|x| (self.func)(x))
    }
}

#[allow(type_alias_bounds)]
type PushBuildImpl<'slf, 'inp, Next, I, F, U>
where
    I: PushBuild,
= I::Build<'slf, 'inp, FlatMap<I::Item, U, impl 'slf + FnMut(I::Item) -> U, Next>>;

impl<I, F, U> PushBuildBase for FlatMapBuild<I, F>
where
    I: PushBuild,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
    type Build<'slf, 'inp, Next>
    where
        Next: Pusherator<Item = Self::Item>,
    = PushBuildImpl<'slf, 'inp, Next, I, F, U>;
}
impl<I, F, U> PushBuild for FlatMapBuild<I, F>
where
    I: PushBuild,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
    type OutputHandoffs = I::OutputHandoffs;

    fn build<'slf, 'inp, Next>(
        &'slf mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'inp>,
        next: Next,
    ) -> Self::Build<'slf, 'inp, Next>
    where
        Next: Pusherator<Item = Self::Item>,
    {
        self.prev
            .build(input, FlatMap::new(|x| (self.func)(x), next))
    }
}
