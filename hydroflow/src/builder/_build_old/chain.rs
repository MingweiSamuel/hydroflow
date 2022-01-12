use super::{BuildBase, PullBuild, PullBuildBase};

use crate::scheduled::handoff::{HandoffList, HandoffListSplit};
use crate::scheduled::type_list::Extend;

pub struct ChainBuild<A, B>
where
    A: PullBuild,
    B: PullBuild<Item = A::Item>,
{
    pull_a: A,
    pull_b: B,
}
impl<A, B> ChainBuild<A, B>
where
    A: PullBuild,
    B: PullBuild<Item = A::Item>,
{
    pub(crate) fn new(pull_a: A, pull_b: B) -> Self {
        Self { pull_a, pull_b }
    }
}

impl<A, B> BuildBase for ChainBuild<A, B>
where
    A: PullBuild,
    B: PullBuild<Item = A::Item>,
{
    type Item = A::Item;
}
impl<A, B> PullBuildBase for ChainBuild<A, B>
where
    A: PullBuild,
    B: PullBuild<Item = A::Item>,
{
    type Build<'slf, 'inp> = std::iter::Chain<A::Build<'slf, 'inp>, B::Build<'slf, 'inp>>;
}
impl<A, B> PullBuild for ChainBuild<A, B>
where
    A: PullBuild,
    B: PullBuild<Item = A::Item>,
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
        iter_a.chain(iter_b)
    }
}
