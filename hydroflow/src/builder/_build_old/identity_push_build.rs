use super::{Push, PushBuild};

use std::marker::PhantomData;

pub struct IdentityPushBuild<T>(PhantomData<fn(T)>);
impl<T> IdentityPushBuild<T> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}
impl<T> PushBuild for IdentityPushBuild<T> {
    type Item = T;

    type Output<O>
    where
        O: Push<Item = Self::Item>,
    = O;
    fn build<O>(self, input: O) -> Self::Output<O>
    where
        O: Push<Item = Self::Item>,
    {
        input
    }
}
