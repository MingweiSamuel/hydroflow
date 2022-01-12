use super::{BuildBase, PullBuild, PullBuildBase, PushBuild, PushBuildBase};

use crate::compiled::map::Map;
use crate::compiled::Pusherator;
use crate::scheduled::handoff::HandoffList;

pub struct MapBuild<I, F>
where
    I: BuildBase,
{
    prev: I,
    func: F,
}
impl<I, F, B> MapBuild<I, F>
where
    I: BuildBase,
    F: FnMut(I::Item) -> B,
{
    pub fn new(prev: I, func: F) -> Self {
        Self { prev, func }
    }
}

impl<I, F, B> BuildBase for MapBuild<I, F>
where
    I: BuildBase,
    F: FnMut(I::Item) -> B,
{
    type Item = B;
}

#[allow(type_alias_bounds)]
type PullBuildImpl<'slf, 'inp, I, F, B>
where
    I: PullBuild,
= std::iter::Map<I::Build<'slf, 'inp>, impl 'slf + FnMut(I::Item) -> B>;

impl<I, F, B> PullBuildBase for MapBuild<I, F>
where
    I: PullBuild,
    F: FnMut(I::Item) -> B,
{
    type Build<'slf, 'inp> = PullBuildImpl<'slf, 'inp, I, F, B>;
}
impl<I, F, B> PullBuild for MapBuild<I, F>
where
    I: PullBuild,
    F: FnMut(I::Item) -> B,
{
    type InputHandoffs = I::InputHandoffs;

    fn build<'slf, 'inp>(
        &'slf mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'inp>,
    ) -> Self::Build<'slf, 'inp> {
        self.prev.build(input).map(|x| (self.func)(x))
    }
}

#[allow(type_alias_bounds)]
type PushBuildImpl<'slf, 'inp, Next, I, F, B>
where
    I: PushBuild,
= I::Build<'slf, 'inp, Map<I::Item, B, impl 'slf + FnMut(I::Item) -> B, Next>>;

impl<I, F, B> PushBuildBase for MapBuild<I, F>
where
    I: PushBuild,
    F: FnMut(I::Item) -> B,
{
    type Build<'slf, 'inp, Next>
    where
        Next: Pusherator<Item = Self::Item>,
    = PushBuildImpl<'slf, 'inp, Next, I, F, B>;
}
impl<I, F, B> PushBuild for MapBuild<I, F>
where
    I: PushBuild,
    F: FnMut(I::Item) -> B,
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
        self.prev.build(input, Map::new(|x| (self.func)(x), next))
    }
}
