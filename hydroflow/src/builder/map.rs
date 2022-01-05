use super::{Pull, PullBase};

use crate::scheduled::handoff::HandoffList;

pub struct MapPull<I, F>
where
    I: Pull,
{
    pull: I,
    func: F,
}
impl<I, F, B> MapPull<I, F>
where
    I: Pull,
    F: FnMut(I::Item) -> B,
{
    pub(crate) fn new(pull: I, func: F) -> Self {
        Self { pull, func }
    }
}

#[allow(type_alias_bounds)]
type Build<'i, I, F, B>
where
    I: Pull,
= std::iter::Map<I::Build<'i>, impl 'i + FnMut(I::Item) -> B>;

impl<I, F, B> PullBase for MapPull<I, F>
where
    I: Pull,
    F: FnMut(I::Item) -> B,
{
    type Item = B;
    type Build<'i> = Build<'i, I, F, B>;
}
impl<I, F, B> Pull for MapPull<I, F>
where
    I: Pull,
    F: FnMut(I::Item) -> B,
{
    type InputHandoffs = I::InputHandoffs;

    fn build(&mut self, input: <Self::InputHandoffs as HandoffList>::RecvCtx<'_>) -> Self::Build<'_> {
        self.pull.build(input).map(|x| (self.func)(x))
    }
}
