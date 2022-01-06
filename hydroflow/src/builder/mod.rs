//! What the builder needs to do:
//! 1. Represent the structure of the graph (chain together exactly like iterators).
//! 2. Construct input and output `TypeList`s and create the subgraph.
//! 3. Connect up handoffs after subgraphs have been created.
//! 4. RUN CODE. (or produce something which runs).

pub mod iterator_factory;

mod chain_pull;
mod flat_map_pull;
mod flat_map_push;
mod for_each_push;
mod handoff_pull;
mod handoff_push;
mod hydroflow_builder;
mod map_pull;
mod map_push;
mod subgraph_build;
mod tee_push;

pub use chain_pull::ChainPull;
pub use flat_map_pull::FlatMapPull;
pub use flat_map_push::FlatMapPush;
pub use for_each_push::ForEachPush;
pub use handoff_pull::HandoffPull;
pub use handoff_push::HandoffPush;
pub use hydroflow_builder::HydroflowBuilder;
pub use map_pull::MapPull;
pub use map_push::MapPush;
pub use subgraph_build::SubgraphBuild;
pub use tee_push::TeePush;

use crate::compiled::Pusherator;
use crate::scheduled::handoff::{CanReceive, Handoff, HandoffList};
use crate::scheduled::type_list::{Extend, Split};

/// If this was directly on [`Pull`], the `Build<'i>` GAT would need an extra
/// `where Self: 'i` bound to prevent a funny edge case when returning `Build<'i> = Self`.
/// This avoids that: https://github.com/rust-lang/rust/issues/87479
pub trait PullBase {
    type Item;
    type Build<'i>: Iterator<Item = Self::Item>;
}
/// Surface API trait for building the pull half of a subgraph.
///
/// This trait is used to:
/// 1. Represent the structure of the graph.
/// 2. Construct the `InputHandoffs` `HandoffList` type for the subgraph.
/// 3. Produce the `Build<'i>` iterator each time the subgraph runs.
/// 4. Provide the surface chaining API!
pub trait Pull: PullBase {
    type InputHandoffs: HandoffList;

    fn init(&mut self, input_ports: <Self::InputHandoffs as HandoffList>::InputPort);

    /// Builds the iterator for a single run of the subgraph.
    fn build(
        &mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'_>,
    ) -> Self::Build<'_>;

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

    // TODO(mingwei): Dedicated FilterMap impl struct.
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

/// Helper, see [`PullBase`] for why this exists, and [`Push`] for where this is used.
pub trait PushBase {
    type Item;
    type Build<'i>: Pusherator<Item = Self::Item>;
}
/// Helper trait for building the push half of a subgraph.
///
/// Unlike [`Push`], this is not the surface API. One more layer is required on
/// the push half in order to reverse the chaining order (push ownership is
/// forward, but chaining naturally builds backwards ownership).
///
/// This trait is used to:
/// 1. Represent the structure of the graph.
/// 2. Construct the `OutputHandoffs` `HandoffList` type for the subgraph.
/// 3. Produce the `Build<'i>` pusherator each time the subgraph runs.
pub trait Push: PushBase {
    type OutputHandoffs: HandoffList;

    fn init(&mut self, output_ports: <Self::OutputHandoffs as HandoffList>::OutputPort);

    fn build<'a>(
        &'a mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'a>,
    ) -> Self::Build<'a>;
}

/// The surface API for the push half of the subgraph.
pub trait PushBuild {
    type Item;

    type Output<O>: PushBuild
    where
        O: Push<Item = Self::Item>;
    fn build<O>(self, input: O) -> Self::Output<O>
    where
        O: Push<Item = Self::Item>;

    fn map<F, C>(self, f: F) -> map_push::MapPushBuild<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> C,
    {
        map_push::MapPushBuild::new(self, f)
    }

    fn flat_map<F, U>(self, f: F) -> flat_map_push::FlatMapPushBuild<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> U,
        U: IntoIterator,
    {
        flat_map_push::FlatMapPushBuild::new(self, f)
    }

    // TODO(mingwei): Dedicated FilterMap impl struct.
    fn filter_map<F, C>(self, f: F) -> flat_map_push::FlatMapPushBuild<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<C>,
    {
        flat_map_push::FlatMapPushBuild::new(self, f)
    }

    fn tee<A, B>(self, a: A, b: B) -> Self::Output<tee_push::TeePush<A, B>>
    where
        Self: Sized,
        A: Push<Item = Self::Item>,
        B: Push<Item = Self::Item>,
        A::Item: Clone,
        A::OutputHandoffs: Extend<B::OutputHandoffs>,
        // The `OutputHandoffs` for chaining (merging two push branches) is just the concatenation of each of their `OutputHandoffs`.
        <A::OutputHandoffs as Extend<B::OutputHandoffs>>::Output: HandoffList,
        // Split trait to split the concatenation back into the two halves. But for the `SendCtx` list rather than the original `HandoffList`.
        for<'a> <<A::OutputHandoffs as Extend<B::OutputHandoffs>>::Output as HandoffList>::SendCtx<'a>:
            Split<
                <A::OutputHandoffs as HandoffList>::SendCtx<'a>,
                <B::OutputHandoffs as HandoffList>::SendCtx<'a>,
            >,
    {
        self.build(tee_push::TeePush::new(a, b))
    }

    fn handoff<H>(self) -> Self::Output<HandoffPush<H, Self::Item>>
    where
        Self: Sized,
        H: Handoff + CanReceive<Self::Item>,
    {
        self.build(HandoffPush::new())
    }

    fn for_each<F>(self, func: F) -> Self::Output<ForEachPush<F, Self::Item>>
    where
        Self: Sized,
        F: FnMut(Self::Item),
    {
        self.build(ForEachPush::new(func))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use ref_cast::RefCast;

    use crate::scheduled::handoff::VecHandoff;
    use crate::tl;

    #[test]
    fn test_chain() {
        use std::cell::RefCell;
        use std::rc::Rc;

        type H = VecHandoff<usize>;
        let pull_a = <HandoffPull<H>>::new().flat_map(|x| x);
        let pull_b = <HandoffPull<H>>::new().flat_map(|x| x);

        let mut pull = pull_a.map(|x| x * x).chain(pull_b);

        let handoff_a = VecHandoff {
            deque: Rc::new(RefCell::new([1, 2, 3, 4].into_iter().collect())),
        };
        let handoff_b = VecHandoff {
            deque: Rc::new(RefCell::new([10, 20, 30, 40].into_iter().collect())),
        };

        let iter = pull.build(tl!(
            RefCast::ref_cast(&handoff_a),
            RefCast::ref_cast(&handoff_b)
        ));
        let output = iter.collect::<Vec<_>>();
        assert_eq!(&[1, 4, 9, 16, 10, 20, 30, 40], &*output);
    }
}
