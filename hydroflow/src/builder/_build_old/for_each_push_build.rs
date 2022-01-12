use super::{PushBuild, PushBuildBase};

use std::marker::PhantomData;

use crate::compiled::for_each::ForEach;
use crate::scheduled::handoff::HandoffList;
use crate::tt;

pub struct ForEachPushBuild<F, T>
where
    F: FnMut(T),
{
    func: F,
    _phantom: PhantomData<fn(T)>,
}
impl<F, T> ForEachPushBuild<F, T>
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
impl<F, T> PushBuildBase for ForEachPushBuild<F, T>
where
    F: FnMut(T),
{
    type Item = T;
    type Build<'a, 'i> = Build<'a, F, T>;
}
impl<F, T> PushBuild for ForEachPushBuild<F, T>
where
    F: FnMut(T),
{
    type OutputHandoffs = tt!();

    fn build<'a, 'i>(
        &'a mut self,
        _input: <Self::OutputHandoffs as HandoffList>::SendCtx<'i>,
    ) -> Self::Build<'a, 'i> {
        ForEach::new(|x| (self.func)(x))
    }
}
