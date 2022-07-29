use std::fmt::Debug;

use crate::{Props, Spec};

pub trait Pusherator {
    type SpecOut<PrevSpec>: Spec
    where
        PrevSpec: Spec;
    type PropsOut<PrevProps>: Props
    where
        PrevProps: Props;

    type ItemIn;

    fn next(&mut self, item: Self::ItemIn);
}

pub struct DebugSink<T>
where
    T: Debug,
{
    _phanton: std::marker::PhantomData<fn(T)>,
}
impl<T> DebugSink<T>
where
    T: Debug,
{
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}
