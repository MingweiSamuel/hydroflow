use super::{Push, PushBase};

use crate::compiled::tee::Tee;
use crate::scheduled::handoff::HandoffList;
use crate::scheduled::type_list::{Extend, Split};

pub struct TeePush<A, B>
where
    A: Push,
    B: Push<Item = A::Item>,
    A::Item: Clone,
{
    push_a: A,
    push_b: B,
}
impl<A, B> TeePush<A, B>
where
    A: Push,
    B: Push<Item = A::Item>,
    A::Item: Clone,
{
    pub(crate) fn new(push_a: A, push_b: B) -> Self {
        Self { push_a, push_b }
    }
}

impl<A, B> PushBase for TeePush<A, B>
where
    A: Push,
    B: Push<Item = A::Item>,
    A::Item: Clone,
{
    type Item = A::Item;
    type Build<'i> = Tee<A::Item, A::Build<'i>, B::Build<'i>>;
}
impl<A, B> Push for TeePush<A, B>
where
    A: Push,
    B: Push<Item = A::Item>,
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
    type OutputHandoffs = <A::OutputHandoffs as Extend<B::OutputHandoffs>>::Output;

    fn init(&mut self, output_ports: <Self::OutputHandoffs as HandoffList>::OutputPort) {
        self.push.init(output_ports)
    }

    fn build<'a>(
        &'a mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'a>,
    ) -> Self::Build<'a> {
        let (input_a, input_b) = input.split();
        let iter_a = self.push_a.build(input_a);
        let iter_b = self.push_b.build(input_b);
        Tee::new(iter_a, iter_b)
    }
}
