use super::{Pull, PullBase};

use crate::scheduled::handoff::HandoffList;

pub struct FilterPull<I, F>
where
    I: Pull,
    F: FnMut(&I::Item) -> bool,
{
    pull: I,
    func: F,
}
impl<I, F> FilterPull<I, F>
where
    I: Pull,
    F: FnMut(&I::Item) -> bool,
{
    pub(crate) fn new(pull: I, func: F) -> Self {
        Self { pull, func }
    }
}

#[allow(type_alias_bounds)]
type Build<'i, I, F>
where
    I: Pull,
    F: FnMut(&I::Item) -> bool,
= std::iter::Filter<I::Build<'i>, impl 'i + FnMut(&I::Item) -> bool>;

impl<I, F> PullBase for FilterPull<I, F>
where
    I: Pull,
    F: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    type Build<'i> = Build<'i, I, F>;
}
impl<I, F> Pull for FilterPull<I, F>
where
    I: Pull,
    F: FnMut(&I::Item) -> bool,
{
    type InputHandoffs = I::InputHandoffs;

    fn init(&mut self, input_ports: <Self::InputHandoffs as HandoffList>::InputPort) {
        self.pull.init(input_ports)
    }

    fn build(
        &mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'_>,
    ) -> Self::Build<'_> {
        self.pull.build(input).filter(|x| (self.func)(x))
    }
}
