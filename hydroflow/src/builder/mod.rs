//! What the builder needs to do:
//! 1. Represent the structure of the graph (chain together exactly like iterators).
//! 2. Construct input and output `TypeList`s and create the subgraph.
//! 3. Connect up handoffs after subgraphs have been created.
//! 4. RUN CODE. (or produce something which runs).

pub mod iterator_factory;

mod map;
pub use map::MapPull;
mod flat_map;
pub use flat_map::FlatMapPull;
mod chain;
pub use chain::ChainPull;

use std::marker::PhantomData;

use crate::scheduled::handoff::{Handoff, HandoffList};
use crate::{tl, tt};

pub trait PullBase {
    type Item;
    type Build<'i>: Iterator<Item = Self::Item>;
}
pub trait Pull: PullBase {
    type InputHandoffs: HandoffList;

    fn build(&mut self, input: <Self::InputHandoffs as HandoffList>::RecvCtx<'_>) -> Self::Build<'_>;

    fn chain<I>(self, pull: I) -> ChainPull<Self, I>
    where
        Self: Sized,
        I: Pull<Item = Self::Item>,
    {
        ChainPull::new(self, pull)
    }

    fn map<F, B>(self, func: F) -> MapPull<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        MapPull::new(self, func)
    }

    fn flat_map<F, U>(self, func: F) -> FlatMapPull<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> U,
        U: IntoIterator,
    {
        FlatMapPull::new(self, func)
    }

    fn filter_map<F, B>(self, func: F) -> FlatMapPull<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        FlatMapPull::new(self, func)
    }

    // fn flatten(self) -> FlatMapPull<Self, std::convert::identity> // expected type, found function `std::convert::identity`
    // where
    //     Self: Sized,
    //     Self::Item: IntoIterator,
    // {
    //     FlatMapPull::new(self, std::convert::identity)
    // }
}

pub struct HandoffPull<H>(PhantomData<*const H>);
impl<H: Handoff> PullBase for HandoffPull<H> {
    type Item = H::Inner;
    type Build<'i> = std::array::IntoIter<Self::Item, 1>;
}
impl<H: Handoff> Pull for HandoffPull<H> {
    type InputHandoffs = tt!(H);

    fn build(&mut self, input: <Self::InputHandoffs as HandoffList>::RecvCtx<'_>) -> Self::Build<'_> {
        let tl!(handoff) = input;
        [handoff.take_inner()].into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chain() {
        use std::rc::Rc;
        use std::cell::RefCell;

        use ref_cast::RefCast;

        use crate::scheduled::handoff::VecHandoff;

        type H = VecHandoff<usize>;
        let pull_a = HandoffPull::<H>(PhantomData).flat_map(|x| x);
        let pull_b = HandoffPull::<H>(PhantomData).flat_map(|x| x);

        let mut pull = pull_a
            .map(|x| x * x)
            .chain(pull_b);

        let handoff_a = VecHandoff {
            deque: Rc::new(RefCell::new([1, 2, 3, 4].into_iter().collect())),
        };
        let handoff_b = VecHandoff {
            deque: Rc::new(RefCell::new([10, 20, 30, 40].into_iter().collect())),
        };

        let iter = pull.build(tl!(RefCast::ref_cast(&handoff_a), RefCast::ref_cast(&handoff_b)));
        let output = iter.collect::<Vec<_>>();
        assert_eq!(&[1, 4, 9, 16, 10, 20, 30, 40], &*output);
    }
}