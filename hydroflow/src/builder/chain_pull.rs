use super::{Pull, PullBase};

use crate::scheduled::handoff::{HandoffList, HandoffListSplit};
use crate::scheduled::type_list::Extend;

pub struct ChainPull<A, B>
where
    A: Pull,
    B: Pull<Item = A::Item>,
{
    pull_a: A,
    pull_b: B,
}
impl<A, B> ChainPull<A, B>
where
    A: Pull,
    B: Pull<Item = A::Item>,
{
    pub(crate) fn new(pull_a: A, pull_b: B) -> Self {
        Self { pull_a, pull_b }
    }
}

impl<A, B> PullBase for ChainPull<A, B>
where
    A: Pull,
    B: Pull<Item = A::Item>,
{
    type Item = A::Item;
    type Build<'i> = std::iter::Chain<A::Build<'i>, B::Build<'i>>;
}
impl<A, B> Pull for ChainPull<A, B>
where
    A: Pull,
    B: Pull<Item = A::Item>,
    A::InputHandoffs: Extend<B::InputHandoffs>,
    <A::InputHandoffs as Extend<B::InputHandoffs>>::Extended:
        HandoffList + HandoffListSplit<A::InputHandoffs, Suffix = B::InputHandoffs>,
{
    type InputHandoffs = <A::InputHandoffs as Extend<B::InputHandoffs>>::Extended;

    fn init(&mut self, input_ports: <Self::InputHandoffs as HandoffList>::InputPort) {
        let (input_ports_a, input_ports_b) =
            <Self::InputHandoffs as HandoffListSplit<_>>::split_input_port(input_ports);
        self.pull_a.init(input_ports_a);
        self.pull_b.init(input_ports_b);
    }

    fn build(
        &mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'_>,
    ) -> Self::Build<'_> {
        let (input_a, input_b) =
            <Self::InputHandoffs as HandoffListSplit<_>>::split_recv_ctx(input);
        let iter_a = self.pull_a.build(input_a);
        let iter_b = self.pull_b.build(input_b);
        iter_a.chain(iter_b)
    }
}
