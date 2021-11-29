use std::cell::RefCell;
use std::collections::VecDeque;

use crate::scheduled::collections::Iter;

use super::{CanReceive, Handoff};

/**
 * A [VecDeque]-based FIFO handoff.
 */
pub struct VecHandoff<T>
where
    T: 'static,
{
    pub(crate) deque: RefCell<VecDeque<T>>,
}
impl<T> Default for VecHandoff<T>
where
    T: 'static,
{
    fn default() -> Self {
        Self {
            deque: Default::default(),
        }
    }
}
impl<T> Handoff for VecHandoff<T> {
    fn is_bottom(&self) -> bool {
        self.deque.borrow_mut().is_empty()
    }

    type Inner = VecDeque<T>;
    fn take_inner(&self) -> Self::Inner {
        self.deque.take()
    }
}

impl<T> CanReceive<Option<T>> for VecHandoff<T> {
    fn give(&self, mut item: Option<T>) -> Option<T> {
        if let Some(item) = item.take() {
            self.deque.borrow_mut().push_back(item)
        }
        None
    }
}
impl<T, I> CanReceive<Iter<I>> for VecHandoff<T>
where
    I: Iterator<Item = T>,
{
    fn give(&self, mut iter: Iter<I>) -> Iter<I> {
        self.deque.borrow_mut().extend(&mut iter.0);
        iter
    }
}
impl<T> CanReceive<VecDeque<T>> for VecHandoff<T> {
    fn give(&self, mut vec: VecDeque<T>) -> VecDeque<T> {
        self.deque.borrow_mut().extend(vec.drain(..));
        vec
    }
}
