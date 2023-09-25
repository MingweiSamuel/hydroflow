//! Traits for the enum `demux` derive and operator.

pub use hydroflow_macro::Demux;
use pusherator::demux::PusheratorList;
use pusherator::Pusherator;
use variadics::Variadic;

/// Trait for use with the enum `demux` operator.
///
/// This trait is meant to be derived: `#[derive(DemuEnum)]`.
pub trait Demux<Nexts>: DemuxItems
where
    Nexts: PusheratorListForItems<Self::Items>,
{
    /// Pushes self into the corresponding output pusherator.
    fn demux(self, outputs: &mut Nexts);
}

/// Fixed output item list for [`DemuxEnum`].
///
/// This trait is meant to be derived: `#[derive(DemuEnum)]`.
pub trait DemuxItems {
    /// A `var_type!(...)` list of items corresponding to each variant's output type.
    type Items: Variadic;
}

/// Helper trait to bound a [`PusheratorList`] variadic to some coresponding item list variadic.
///
/// A pusherator list `var_type!(PushA, PushB, PushC)` implements `PusheratorListForItems<var_type!(ItemA, ItemB, ItemC)>`,
/// where `PushA: Pusherator<Item = ItemA>`, etc.
pub trait PusheratorListForItems<Items>: PusheratorList
where
    Items: Variadic,
{
}
impl<HeadPush, RestPush, Head, Rest> PusheratorListForItems<(Head, Rest)> for (HeadPush, RestPush)
where
    HeadPush: Pusherator<Item = Head>,
    RestPush: PusheratorListForItems<Rest>,
    Rest: Variadic,
{
}
impl PusheratorListForItems<()> for () {}
