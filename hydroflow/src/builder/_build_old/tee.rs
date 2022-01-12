use super::{BuildBase, PushBuild, PushFinalBuild, PushFinalBuildBase};

use crate::compiled::tee::Tee;
use crate::scheduled::handoff::{HandoffList, HandoffListSplit};
use crate::scheduled::type_list::Extend;

pub struct TeePush<I, A, B>
where
    I: PushBuild,
    A: PushFinalBuild,
    B: PushFinalBuild<InItem = A::InItem>,
    A::InItem: Clone,
{
    prev: I,
    push_a: A,
    push_b: B,
}
impl<I, A, B> TeePush<I, A, B>
where
    I: PushBuild,
    A: PushFinalBuild,
    B: PushFinalBuild<InItem = A::InItem>,
    A::InItem: Clone,
{
    pub fn new(prev: I, push_a: A, push_b: B) -> Self {
        Self {
            prev,
            push_a,
            push_b,
        }
    }
}

impl<I, A, B> BuildBase for TeePush<I, A, B>
where
    I: PushBuild,
    A: PushFinalBuild,
    B: PushFinalBuild<InItem = A::InItem>,
    A::InItem: Clone,
{
    type Item = A::InItem;
}

impl<I, A, B> PushFinalBuildBase for TeePush<I, A, B>
where
    I: PushBuild<Item = A::InItem>,
    A: PushFinalBuild,
    B: PushFinalBuild<InItem = A::InItem>,
    A::InItem: Clone,
{
    type InItem = A::InItem;
    type Build<'slf, 'inp> =
        I::Build<'slf, 'inp, Tee<Self::InItem, A::Build<'slf, 'inp>, B::Build<'slf, 'inp>>>;
}

impl<I, A, B> PushFinalBuild for TeePush<I, A, B>
where
    I: PushBuild<Item = A::InItem>,
    A: PushFinalBuild,
    B: PushFinalBuild<InItem = A::InItem>,
    A::InItem: Clone,
    A::OutputHandoffs: Extend<B::OutputHandoffs>,
    <A::OutputHandoffs as Extend<B::OutputHandoffs>>::Extended:
        HandoffList + HandoffListSplit<A::OutputHandoffs, Suffix = B::OutputHandoffs>,
{
    type OutputHandoffs = <A::OutputHandoffs as Extend<B::OutputHandoffs>>::Extended;

    fn build<'slf, 'inp>(
        &'slf mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'inp>,
    ) -> Self::Build<'slf, 'inp> {
        let (input_a, input_b) =
            <Self::OutputHandoffs as HandoffListSplit<_>>::split_send_ctx(input);
        let iter_a = self.push_a.build(input_a);
        let iter_b = self.push_b.build(input_b);
        self.prev.build(Tee::new(iter_a, iter_b))
    }
}
