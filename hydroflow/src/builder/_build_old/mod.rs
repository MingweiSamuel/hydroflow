pub mod chain;
pub mod filter;
pub mod flat_map;
pub mod join;
pub mod map;
pub mod tee;

// pub mod chain_pull_build;
// pub mod filter_pull_build;
// pub mod flat_map_pull_build;
// pub mod join_pull_build;
// pub mod map_pull_build;

// // pub mod flat_map_push_build;
// // pub mod for_each_push_build;
// // pub mod identity_push_build;
// pub mod map_push_build;
// // pub mod tee_push_build;

use crate::compiled::Pusherator;
use crate::scheduled::handoff::HandoffList;

/// Shared root trat between [PullBuild] and [PushBuild] for convenience.
pub trait BuildBase {
    type Item;
}

/// If this was directly on [`PullBase`], the `Build<'i>` GAT would need an extra
/// `where Self: 'i` bound to prevent a funny edge case when returning `Build<'i> = Self`.
/// This avoids that: <https://github.com/rust-lang/rust/issues/87479>
pub trait PullBuildBase: BuildBase {
    type Build<'slf, 'inp>: Iterator<Item = Self::Item>;
}
pub trait PullBuild: PullBuildBase {
    type InputHandoffs: HandoffList;

    /// Builds the iterator for a single run of the subgraph.
    fn build<'slf, 'inp>(
        &'slf mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'inp>,
    ) -> Self::Build<'slf, 'inp>;
}

/// TODO DOC: FOR INTERMEDIATE OPS
pub trait PushBuildBase: BuildBase {
    type Build<'slf, 'inp, Next>: Pusherator
    where
        Next: Pusherator<Item = Self::Item>;
}
pub trait PushBuild: PushBuildBase {
    type OutputHandoffs: HandoffList;

    fn build<'slf, 'inp, Next>(
        &'slf mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'inp>,
        next: Next,
    ) -> Self::Build<'slf, 'inp, Next>
    where
        Next: Pusherator<Item = Self::Item>;
}

/// TODO DOC: FOR FINAL OPS
pub trait PushFinalBuildBase {
    type InItem;
    type Build<'slf, 'inp>: Pusherator<Item = Self::InItem>;
}
pub trait PushFinalBuild: PushFinalBuildBase {
    type OutputHandoffs: HandoffList;

    fn build<'slf, 'inp>(
        &'slf mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'inp>,
    ) -> Self::Build<'slf, 'inp>;
}
