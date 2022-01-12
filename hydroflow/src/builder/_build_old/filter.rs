use super::{BuildBase, PullBuild, PullBuildBase, PushBuild, PushBuildBase};

use crate::compiled::filter::Filter;
use crate::compiled::Pusherator;
use crate::scheduled::handoff::HandoffList;

pub struct FilterBuild<I, F>
where
    I: BuildBase,
{
    prev: I,
    func: F,
}
impl<I, F> FilterBuild<I, F>
where
    I: BuildBase,
    F: FnMut(&I::Item) -> bool,
{
    pub fn new(prev: I, func: F) -> Self {
        Self { prev, func }
    }
}

impl<I, F> BuildBase for FilterBuild<I, F>
where
    I: BuildBase,
    F: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
}

#[allow(type_alias_bounds)]
type PullBuildImpl<'slf, 'inp, I, F>
where
    I: PullBuild,
= std::iter::Filter<I::Build<'slf, 'inp>, impl 'slf + FnMut(&I::Item) -> bool>;

impl<I, F> PullBuildBase for FilterBuild<I, F>
where
    I: PullBuild,
    F: FnMut(&I::Item) -> bool,
{
    type Build<'slf, 'inp> = PullBuildImpl<'slf, 'inp, I, F>;
}
impl<I, F> PullBuild for FilterBuild<I, F>
where
    I: PullBuild,
    F: FnMut(&I::Item) -> bool,
{
    type InputHandoffs = I::InputHandoffs;

    fn build<'slf, 'inp>(
        &'slf mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'inp>,
    ) -> Self::Build<'slf, 'inp> {
        self.prev.build(input).filter(|x| (self.func)(x))
    }
}

#[allow(type_alias_bounds)]
type PushBuildImpl<'slf, 'inp, Next, I, F>
where
    I: PushBuild,
= I::Build<'slf, 'inp, Filter<I::Item, impl 'slf + FnMut(&I::Item) -> bool, Next>>;

impl<I, F> PushBuildBase for FilterBuild<I, F>
where
    I: PushBuild,
    F: FnMut(&I::Item) -> bool,
{
    type Build<'slf, 'inp, Next>
    where
        Next: Pusherator<Item = Self::Item>,
    = PushBuildImpl<'slf, 'inp, Next, I, F>;
}
impl<I, F> PushBuild for FilterBuild<I, F>
where
    I: PushBuild,
    F: FnMut(&I::Item) -> bool,
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
            .build(input, Filter::new(|x| (self.func)(x), next))
    }
}
