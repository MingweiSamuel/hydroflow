use super::{BuildBase, PullBuild, PullBuildBase};

use std::hash::Hash;

use crate::compiled::pull::{JoinState, SymmetricHashJoin};
use crate::scheduled::handoff::{HandoffList, HandoffListSplit};
use crate::scheduled::type_list::Extend;

pub struct JoinBuild<A, B, K, VA, VB>
where
    A: PullBuild<Item = (K, VA)>,
    B: PullBuild<Item = (K, VB)>,
    K: 'static + Eq + Hash + Clone,
    VA: 'static + Eq + Clone,
    VB: 'static + Eq + Clone,
{
    pull_a: A,
    pull_b: B,
    state: JoinState<K, VA, VB>,
}
impl<A, B, K, VA, VB> JoinBuild<A, B, K, VA, VB>
where
    A: PullBuild<Item = (K, VA)>,
    B: PullBuild<Item = (K, VB)>,
    K: 'static + Eq + Hash + Clone,
    VA: 'static + Eq + Clone,
    VB: 'static + Eq + Clone,
{
    pub fn new(pull_a: A, pull_b: B) -> Self {
        Self {
            pull_a,
            pull_b,
            state: Default::default(),
        }
    }
}

impl<A, B, K, VA, VB> BuildBase for JoinBuild<A, B, K, VA, VB>
where
    A: PullBuild<Item = (K, VA)>,
    B: PullBuild<Item = (K, VB)>,
    K: 'static + Eq + Hash + Clone,
    VA: 'static + Eq + Clone,
    VB: 'static + Eq + Clone,
{
    type Item = (K, VA, VB);
}

impl<A, B, K, VA, VB> PullBuildBase for JoinBuild<A, B, K, VA, VB>
where
    A: PullBuild<Item = (K, VA)>,
    B: PullBuild<Item = (K, VB)>,
    K: 'static + Eq + Hash + Clone,
    VA: 'static + Eq + Clone,
    VB: 'static + Eq + Clone,
{
    type Build<'slf, 'inp> =
        SymmetricHashJoin<'slf, K, A::Build<'slf, 'inp>, VA, B::Build<'slf, 'inp>, VB>;
}

impl<A, B, K, VA, VB> PullBuild for JoinBuild<A, B, K, VA, VB>
where
    A: PullBuild<Item = (K, VA)>,
    B: PullBuild<Item = (K, VB)>,
    K: 'static + Eq + Hash + Clone,
    VA: 'static + Eq + Clone,
    VB: 'static + Eq + Clone,

    A::InputHandoffs: Extend<B::InputHandoffs>,
    <A::InputHandoffs as Extend<B::InputHandoffs>>::Extended:
        HandoffList + HandoffListSplit<A::InputHandoffs, Suffix = B::InputHandoffs>,
{
    type InputHandoffs = <A::InputHandoffs as Extend<B::InputHandoffs>>::Extended;

    fn build<'slf, 'inp>(
        &'slf mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'inp>,
    ) -> Self::Build<'slf, 'inp> {
        let (input_a, input_b) =
            <Self::InputHandoffs as HandoffListSplit<_>>::split_recv_ctx(input);
        let iter_a = self.pull_a.build(input_a);
        let iter_b = self.pull_b.build(input_b);
        SymmetricHashJoin::new(iter_a, iter_b, &mut self.state)
    }
}
