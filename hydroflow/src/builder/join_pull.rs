use super::{Pull, PullBase};

use std::hash::Hash;

use crate::compiled::pull::{JoinState, SymmetricHashJoin};
use crate::scheduled::handoff::{HandoffList, HandoffListSplit};
use crate::scheduled::type_list::Extend;

pub struct JoinPull<A, B, K, VA, VB>
where
    A: Pull<Item = (K, VA)>,
    B: Pull<Item = (K, VB)>,
    K: 'static + Eq + Hash + Clone,
    VA: 'static + Eq + Clone,
    VB: 'static + Eq + Clone,
{
    pull_a: A,
    pull_b: B,
    state: JoinState<K, VA, VB>,
}
impl<A, B, K, VA, VB> JoinPull<A, B, K, VA, VB>
where
    A: Pull<Item = (K, VA)>,
    B: Pull<Item = (K, VB)>,
    K: 'static + Eq + Hash + Clone,
    VA: 'static + Eq + Clone,
    VB: 'static + Eq + Clone,
{
    pub(crate) fn new(pull_a: A, pull_b: B) -> Self {
        Self {
            pull_a,
            pull_b,
            state: Default::default(),
        }
    }
}

impl<A, B, K, VA, VB> PullBase for JoinPull<A, B, K, VA, VB>
where
    A: Pull<Item = (K, VA)>,
    B: Pull<Item = (K, VB)>,
    K: 'static + Eq + Hash + Clone,
    VA: 'static + Eq + Clone,
    VB: 'static + Eq + Clone,
{
    type Item = (K, VA, VB);
    type Build<'i> = SymmetricHashJoin<'i, K, A::Build<'i>, VA, B::Build<'i>, VB>;
}

impl<A, B, K, VA, VB> Pull for JoinPull<A, B, K, VA, VB>
where
    A: Pull<Item = (K, VA)>,
    B: Pull<Item = (K, VB)>,
    K: 'static + Eq + Hash + Clone,
    VA: 'static + Eq + Clone,
    VB: 'static + Eq + Clone,

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
        SymmetricHashJoin::new(iter_a, iter_b, &mut self.state)
    }
}
