use super::{PullBuild, PullBuildBase};

use std::hash::Hash;

use crate::compiled::pull::{JoinState, SymmetricHashJoin};
use crate::scheduled::handoff::{HandoffList, HandoffListSplit};
use crate::scheduled::type_list::Extend;

pub struct JoinPullBuild<ItemA, ItemB, Key, ValA, ValB>
where
    ItemA: PullBuild<ItemOut = (Key, ValA)>,
    ItemB: PullBuild<ItemOut = (Key, ValB)>,
    Key: 'static + Eq + Hash + Clone,
    ValA: 'static + Eq + Clone,
    ValB: 'static + Eq + Clone,
{
    prev_a: ItemA,
    prev_b: ItemB,
    state: JoinState<Key, ValA, ValB>,
}
impl<ItemA, ItemB, Key, ValA, ValB> JoinPullBuild<ItemA, ItemB, Key, ValA, ValB>
where
    ItemA: PullBuild<ItemOut = (Key, ValA)>,
    ItemB: PullBuild<ItemOut = (Key, ValB)>,
    Key: 'static + Eq + Hash + Clone,
    ValA: 'static + Eq + Clone,
    ValB: 'static + Eq + Clone,
{
    pub fn new(prev_a: ItemA, prev_b: ItemB) -> Self {
        Self {
            prev_a,
            prev_b,
            state: Default::default(),
        }
    }
}

impl<ItemA, ItemB, Key, ValA, ValB> PullBuildBase for JoinPullBuild<ItemA, ItemB, Key, ValA, ValB>
where
    ItemA: PullBuild<ItemOut = (Key, ValA)>,
    ItemB: PullBuild<ItemOut = (Key, ValB)>,
    Key: 'static + Eq + Hash + Clone,
    ValA: 'static + Eq + Clone,
    ValB: 'static + Eq + Clone,
{
    type ItemOut = (Key, ValA, ValB);
    type Build<'slf, 'hof> = SymmetricHashJoin<
        'slf,
        Key,
        ItemA::Build<'slf, 'hof>,
        ValA,
        ItemB::Build<'slf, 'hof>,
        ValB,
    >;
}

impl<ItemA, ItemB, Key, ValA, ValB> PullBuild for JoinPullBuild<ItemA, ItemB, Key, ValA, ValB>
where
    ItemA: PullBuild<ItemOut = (Key, ValA)>,
    ItemB: PullBuild<ItemOut = (Key, ValB)>,
    Key: 'static + Eq + Hash + Clone,
    ValA: 'static + Eq + Clone,
    ValB: 'static + Eq + Clone,

    ItemA::InputHandoffs: Extend<ItemB::InputHandoffs>,
    <ItemA::InputHandoffs as Extend<ItemB::InputHandoffs>>::Extended:
        HandoffList + HandoffListSplit<ItemA::InputHandoffs, Suffix = ItemB::InputHandoffs>,
{
    type InputHandoffs = <ItemA::InputHandoffs as Extend<ItemB::InputHandoffs>>::Extended;

    fn build<'slf, 'hof>(
        &'slf mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'hof>,
    ) -> Self::Build<'slf, 'hof> {
        let (input_a, input_b) =
            <Self::InputHandoffs as HandoffListSplit<_>>::split_recv_ctx(input);
        let iter_a = self.prev_a.build(input_a);
        let iter_b = self.prev_b.build(input_b);
        SymmetricHashJoin::new(iter_a, iter_b, &mut self.state)
    }
}
