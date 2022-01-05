use super::{Push, PushBase, PushBuild};

use std::marker::PhantomData;

use crate::compiled::map::Map;
use crate::scheduled::handoff::HandoffList;

pub struct MapPush<I, F, B>
where
    I: Push,
    F: FnMut(B) -> I::Item,
{
    push: I,
    func: F,
    _phantom: PhantomData<fn(B) -> I::Item>,
}
impl<I, F, B> MapPush<I, F, B>
where
    I: Push,
    F: FnMut(B) -> I::Item,
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
type Build<'a, 'i, I, F, B>
where
    I: Push,
= Map<B, I::Item, impl 'a + FnMut(B) -> I::Item, I::Build<'a, 'i>>;

impl<I, F, B> PushBase for MapPush<I, F, B>
where
    I: Push,
    F: FnMut(B) -> I::Item,
{
    type Item = B;
    type Build<'a, 'i> = Build<'a, 'i, I, F, B>;
}
impl<I, F, B> Push for MapPush<I, F, B>
where
    I: Push,
    F: FnMut(B) -> I::Item,
{
    type OutputHandoffs = I::OutputHandoffs;

    fn init(&mut self, output_ports: <Self::OutputHandoffs as HandoffList>::OutputPort) {
        self.push.init(output_ports)
    }

    fn build<'a, 'i>(
        &'a mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'i>,
    ) -> Self::Build<'a, 'i> {
        Map::new(|x| (self.func)(x), self.push.build(input))
    }
}

pub struct MapPushBuild<P, F>
where
    P: PushBuild,
{
    prev: P,
    func: F,
}
impl<P, F, C> MapPushBuild<P, F>
where
    P: PushBuild,
    F: FnMut(P::Item) -> C,
{
    pub(crate) fn new(prev: P, func: F) -> Self {
        Self { prev, func }
    }
}
impl<P, F, C> PushBuild for MapPushBuild<P, F>
where
    P: PushBuild,
    F: FnMut(P::Item) -> C,
{
    type Item = C;

    type Output<O>
    where
        O: Push<Item = Self::Item>,
    = P::Output<MapPush<O, F, P::Item>>;
    fn build<O>(self, input: O) -> Self::Output<O>
    where
        O: Push<Item = Self::Item>,
    {
        self.prev.build(MapPush::new(input, self.func))
    }
}
