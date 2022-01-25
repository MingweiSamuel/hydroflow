#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

use hydroflow::builder::build::{PullBuild, PullBuildBase, PushBuild, PushBuildBase};
use hydroflow::builder::surface::{BaseSurface, PullSurface, PushSurface, PushSurfaceReversed};
use hydroflow::scheduled::handoff::HandoffList;
use hydroflow::compiled::Pusherator;

hydroflow_macro::surface_unary! {
    B =>
    fn map<B, F>(self, f: F) -> Map<Self, F>
    where
        F: FnMut(Self::Item) -> B;
}

// hydroflow_macro::surface_unary! {
//     Prev::ItemOut =>
//     fn filter<P>(self, predicate: P) -> Filter<Self, P>
//     where
//         P: FnMut(&Self::Item) -> bool;
// }

// hydroflow_macro::surface_unary! {
//     B =>
//     fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
//     where
//         F: FnMut(Self::Item) -> Option<B>;
// }
