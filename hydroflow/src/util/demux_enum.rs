//! Traits for the `demux_enum` derive and operator.

pub use hydroflow_macro::DemuxEnum;
use pusherator::demux::PusheratorList;
use pusherator::Pusherator;
use variadics::Variadic;

/// Trait for use with the `demux_enum` operator.
///
/// This trait is meant to be derived: `#[derive(DemuEnum)]`.
pub trait DemuxEnum<Nexts>: DemuxEnumItems
where
    Nexts: PusheratorListForItems<Self::Items>,
{
    /// Pushes self into the corresponding output pusherator.
    fn demux_enum(self, outputs: &mut Nexts);
}

/// Fixed output item list for [`DemuxEnum`].
///
/// This trait is meant to be derived: `#[derive(DemuEnum)]`.
pub trait DemuxEnumItems {
    /// A `Var!(...)` list of items corresponding to each variant's output type.
    type Items: Variadic;
}

/// Helper trait to bound a [`PusheratorList`] variadic to some coresponding item list variadic.
///
/// A pusherator list `Var!(PushA, PushB, PushC)` implements `PusheratorListForItems<Var!(ItemA, ItemB, ItemC)>`,
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
