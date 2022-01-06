use super::{Push, PushBase, PushBuild};

use std::marker::PhantomData;

use crate::compiled::flat_map::FlatMap;
use crate::scheduled::handoff::HandoffList;

pub struct FlatMapPush<I, F, B>
where
    I: Push,
{
    push: I,
    func: F,
    _phantom: PhantomData<fn(B)>,
}
impl<I, F, B, U> FlatMapPush<I, F, B>
where
    I: Push,
    F: FnMut(B) -> U,
    U: IntoIterator<Item = I::Item>,
{
    pub(crate) fn new(push: I, func: F) -> Self {
        Self {
            push,
            func,
            _phantom: PhantomData,
        }
    }
}

#[allow(type_alias_bounds)]
type Build<'i, I, F, B, U>
where
    I: Push,
    U: IntoIterator<Item = I::Item>,
= FlatMap<B, U, impl 'i + FnMut(B) -> U, I::Build<'i>>;

impl<I, F, B, U> PushBase for FlatMapPush<I, F, B>
where
    I: Push,
    F: FnMut(B) -> U,
    U: IntoIterator<Item = I::Item>,
{
    type Item = B;
    type Build<'i> = Build<'i, I, F, B, U>;
}
impl<I, F, B, U> Push for FlatMapPush<I, F, B>
where
    I: Push,
    F: FnMut(B) -> U,
    U: IntoIterator<Item = I::Item>,
{
    type OutputHandoffs = I::OutputHandoffs;

    fn init(&mut self, output_ports: <Self::OutputHandoffs as HandoffList>::OutputPort) {
        self.push.init(output_ports)
    }

    fn build<'a>(
        &'a mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'a>,
    ) -> Self::Build<'a> {
        FlatMap::new(|x| (self.func)(x), self.push.build(input))
    }
}

pub struct FlatMapPushBuild<P, F>
where
    P: PushBuild,
{
    prev: P,
    func: F,
}
impl<P, F, U> FlatMapPushBuild<P, F>
where
    P: PushBuild,
    F: FnMut(P::Item) -> U,
    U: IntoIterator,
{
    pub(crate) fn new(prev: P, func: F) -> Self {
        Self { prev, func }
    }
}
impl<P, F, U> PushBuild for FlatMapPushBuild<P, F>
where
    P: PushBuild,
    F: FnMut(P::Item) -> U,
    U: IntoIterator,
{
    type Item = U::Item;

    type Output<O>
    where
        O: Push<Item = Self::Item>,
    = P::Output<FlatMapPush<O, F, P::Item>>;
    fn build<O>(self, input: O) -> Self::Output<O>
    where
        O: Push<Item = Self::Item>,
    {
        self.prev.build(FlatMapPush::new(input, self.func))
    }
}
