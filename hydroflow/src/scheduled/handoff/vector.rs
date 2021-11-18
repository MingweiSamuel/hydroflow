use std::collections::VecDeque;
use std::marker::PhantomData;

use crate::scheduled::collections::Iter;

use super::{CanReceive, Handoff};

/**
 * A [VecDeque]-based FIFO handoff.
 */
pub struct VecHandoff<T>
where
    T: 'static,
{
    _phantom: PhantomData<*mut T>,
}
impl<T> Handoff for VecHandoff<T> {
    type State = VecDeque<T>;

    fn is_bottom(state: &Self::State) -> bool {
        state.is_empty()
    }

    type Inner = VecDeque<T>;
    fn take_inner(state: &mut Self::State) -> Self::Inner {
        std::mem::take(state)
    }
}

impl<T> CanReceive<Option<T>> for VecHandoff<T> {
    fn give(state: &mut Self::State, mut item: Option<T>) -> Option<T> {
        if let Some(item) = item.take() {
            state.push_back(item)
        }
        None
    }
}
impl<T, I> CanReceive<Iter<I>> for VecHandoff<T>
where
    I: Iterator<Item = T>,
{
    fn give(state: &mut Self::State, mut iter: Iter<I>) -> Iter<I> {
        state.extend(&mut iter.0);
        iter
    }
}
impl<T> CanReceive<VecDeque<T>> for VecHandoff<T> {
    fn give(state: &mut Self::State, mut vec: VecDeque<T>) -> VecDeque<T> {
        state.extend(vec.drain(..));
        vec
    }
}
