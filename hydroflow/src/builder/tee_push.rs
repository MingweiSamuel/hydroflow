use super::{Push, PushBase};

use crate::compiled::tee::Tee;
use crate::scheduled::handoff::{HandoffList, HandoffListSplit};
use crate::scheduled::type_list::Extend;

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
    type Build<'a, 'i> = Tee<A::Item, A::Build<'a, 'i>, B::Build<'a, 'i>>;
}
impl<A, B> Push for TeePush<A, B>
where
    A: Push,
    B: Push<Item = A::Item>,
    A::Item: Clone,
    A::OutputHandoffs: Extend<B::OutputHandoffs>,
    // Needed to un-concat the handoff lists.
    <A::OutputHandoffs as Extend<B::OutputHandoffs>>::Output:
        HandoffList + HandoffListSplit<A::OutputHandoffs, B::OutputHandoffs>,
{
    type OutputHandoffs = <A::OutputHandoffs as Extend<B::OutputHandoffs>>::Output;

    fn init(&mut self, output_ports: <Self::OutputHandoffs as HandoffList>::OutputPort) {
        let (output_ports_a, output_ports_b) = <Self::OutputHandoffs as HandoffListSplit<
            A::OutputHandoffs,
            B::OutputHandoffs,
        >>::split_output_port(output_ports);
        self.push_a.init(output_ports_a);
        self.push_b.init(output_ports_b);
    }

    fn build<'a, 'i>(
        &'a mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'i>,
    ) -> Self::Build<'a, 'i> {
        let (input_a, input_b) = <Self::OutputHandoffs as HandoffListSplit<
            A::OutputHandoffs,
            B::OutputHandoffs,
        >>::split_send_ctx(input);
        let iter_a = self.push_a.build(input_a);
        let iter_b = self.push_b.build(input_b);
        Tee::new(iter_a, iter_b)
    }
}
