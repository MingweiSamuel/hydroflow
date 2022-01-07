use super::{Pull, PullBase};

use std::hash::Hash;

use crate::compiled::pull::{JoinState, SymmetricHashJoin};
use crate::scheduled::handoff::HandoffList;
use crate::scheduled::type_list::{Extend, Split};

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
    // The `InputHandoffs` for chaining (merging two pull branches) is just the concatenation of each of their `InputHandoffs`.
    <A::InputHandoffs as Extend<B::InputHandoffs>>::Output: HandoffList,
    // Split trait to split the concatenation back into the two halves. But for the `InputPort` list.
    <<A::InputHandoffs as Extend<B::InputHandoffs>>::Output as HandoffList>::InputPort: Split<
        <A::InputHandoffs as HandoffList>::InputPort,
        <B::InputHandoffs as HandoffList>::InputPort,
    >,
    // Split trait to split the concatenation back into the two halves. But for the `RecvCtx` list rather than the original `HandoffList`.
    for<'a> <<A::InputHandoffs as Extend<B::InputHandoffs>>::Output as HandoffList>::RecvCtx<'a>:
        Split<
            <A::InputHandoffs as HandoffList>::RecvCtx<'a>,
            <B::InputHandoffs as HandoffList>::RecvCtx<'a>,
        >,
{
    type InputHandoffs = <A::InputHandoffs as Extend<B::InputHandoffs>>::Output;

    fn init(&mut self, input_ports: <Self::InputHandoffs as HandoffList>::InputPort) {
        let (input_ports_a, input_ports_b) = input_ports.split();
        self.pull_a.init(input_ports_a);
        self.pull_b.init(input_ports_b);
    }

    fn build(
        &mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'_>,
    ) -> Self::Build<'_> {
        let (input_a, input_b) = input.split();
        let iter_a = self.pull_a.build(input_a);
        let iter_b = self.pull_b.build(input_b);
        SymmetricHashJoin::new(iter_a, iter_b, &mut self.state)
    }
}
