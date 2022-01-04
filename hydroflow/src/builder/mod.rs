pub mod iterator_factory;

use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::scheduled::ctx::RecvCtx;
use crate::scheduled::graph::Hydroflow;
use crate::scheduled::handoff::Handoff;

pub struct BuildContext {
    hydroflow: Hydroflow,
    inputs: HashMap<&'static str, Box<dyn Any>>,
}
impl BuildContext {
    // pub fn make_handoff<H>(&mut self) -> (BuildHandoffInput<H>, BuildHandoffOutput<H>)
    // where
    //     H: Handoff,
    // {
    // }
}

pub struct BuildHandoffInput<'a, H>
where
    H: Handoff,
{
    build_context: &'a mut BuildContext,
    _phantom: PhantomData<*const H>,
}

pub struct BuildHandoffOutput<'a, H>
where
    H: Handoff,
{
    build_context: &'a mut BuildContext,
    _phantom: PhantomData<*const H>,
}
impl<'a, H> iterator_factory::IteratorFactorySuper for BuildHandoffOutput<'a, H>
where
    H: Handoff,
{
    type Item = H::Inner;
    type Iterator<'i> = std::array::IntoIter<H::Inner, 1>;
}
impl<'a, H> iterator_factory::IteratorFactory for BuildHandoffOutput<'a, H>
where
    H: Handoff,
{
    type Input = RecvCtx<H>;
    fn build_iterator(&mut self, input: Self::Input) -> Self::Iterator<'_> {
        [input.take_inner()].into_iter()
    }
}

pub fn build_hydroflow<F>(build_fn: F) -> Hydroflow
where
    F: FnOnce(&mut BuildContext),
{
    todo!()
}
