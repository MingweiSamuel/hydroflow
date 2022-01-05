use super::{Pull, PullBase};

use crate::scheduled::handoff::HandoffList;

pub struct FlatMapPull<I, F>
where
    I: Pull,
{
    pull: I,
    func: F,
}
impl<I, F, U> FlatMapPull<I, F>
where
    I: Pull,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
    pub(crate) fn new(pull: I, func: F) -> Self {
        Self { pull, func }
    }
}

#[allow(type_alias_bounds)]
type Build<'i, I, F, U>
where
    I: Pull,
= std::iter::FlatMap<I::Build<'i>, U, impl 'i + FnMut(I::Item) -> U>;

impl<I, F, U> PullBase for FlatMapPull<I, F>
where
    I: Pull,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
    type Item = U::Item;
    type Build<'i> = Build<'i, I, F, U>;
}
impl<I, F, U> Pull for FlatMapPull<I, F>
where
    I: Pull,
    F: FnMut(I::Item) -> U,
    U: IntoIterator,
{
    type InputHandoffs = I::InputHandoffs;

    fn build(&mut self, input: <Self::InputHandoffs as HandoffList>::RecvCtx<'_>) -> Self::Build<'_> {
        self.pull.build(input).flat_map(|x| (self.func)(x))
    }
}
