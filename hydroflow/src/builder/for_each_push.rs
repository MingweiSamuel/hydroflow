use super::{Push, PushBase};

use std::marker::PhantomData;

use crate::compiled::for_each::ForEach;
use crate::scheduled::handoff::HandoffList;
use crate::tt;

pub struct ForEachPush<F, T>
where
    F: FnMut(T),
{
    func: F,
    _phantom: PhantomData<fn(T)>,
}
impl<F, T> ForEachPush<F, T>
where
    F: FnMut(T),
{
    pub(crate) fn new(func: F) -> Self {
        Self {
            func,
            _phantom: PhantomData,
        }
    }
}

#[allow(type_alias_bounds)]
type Build<'a, F, T> = ForEach<T, impl 'a + FnMut(T)>;
impl<F, T> PushBase for ForEachPush<F, T>
where
    F: FnMut(T),
{
    type Item = T;
    type Build<'a, 'i> = Build<'a, F, T>;
}
impl<F, T> Push for ForEachPush<F, T>
where
    F: FnMut(T),
{
    type OutputHandoffs = tt!();

    fn init(&mut self, _output_ports: <Self::OutputHandoffs as HandoffList>::OutputPort) {}

    fn build<'a, 'i>(
        &'a mut self,
        _input: <Self::OutputHandoffs as HandoffList>::SendCtx<'i>,
    ) -> Self::Build<'a, 'i> {
        ForEach::new(|x| (self.func)(x))
    }
}
