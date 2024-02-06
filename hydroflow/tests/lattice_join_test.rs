// use std::collections::hash_map::Entry::*;

// use hydroflow::compiled::pull::HalfJoinState;
// use rustc_hash::FxHashMap;
// use smallvec::{smallvec, SmallVec};

// #[derive(Default)]
// pub struct HalfJoinStateLattice<Map> {
//     pub map: Map,
// }

// impl<Map, Key, ValBuild, ValProbe> HalfJoinState<Key, ValBuild, ValProbe>
//     for HalfJoinStateLattice<Map>
// where
//     Map: hydroflow::lattices::cc_traits::Map<Key, ValBuild, Key = Key, Item = ValBuild>,
//     // Key: Clone + Eq + std::hash::Hash,
//     // ValBuild: Clone,
//     // ValProbe: Clone,
// {
//     fn build(&mut self, k: Key, v: &ValBuild) -> bool {
//         let entry = self.table.entry(k);

//         match entry {
//             Entry::Occupied(mut e) => {
//                 let vec = e.get_mut();

//                 vec.push(v.clone());
//                 self.len += 1;
//             }
//             Entry::Vacant(e) => {
//                 e.insert(smallvec![v.clone()]);
//                 self.len += 1;
//             }
//         };

//         true
//     }

//     fn probe(&mut self, k: &Key, v: &ValProbe) -> Option<(Key, ValProbe, ValBuild)> {
//         // TODO: We currently don't free/shrink the self.current_matches vecdeque to save time.
//         // This mean it will grow to eventually become the largest number of matches in a single probe call.
//         // Maybe we should clear this memory at the beginning of every tick/periodically?
//         let mut iter = self
//             .table
//             .get(k)?
//             .iter()
//             .map(|valbuild| (k.clone(), v.clone(), valbuild.clone()));

//         let first = iter.next();

//         self.current_matches.extend(iter);

//         first
//     }

//     fn full_probe(&self, k: &Key) -> slice::Iter<'_, ValBuild> {
//         let Some(sv) = self.table.get(k) else {
//             return [].iter();
//         };

//         sv.iter()
//     }

//     fn pop_match(&mut self) -> Option<(Key, ValProbe, ValBuild)> {
//         self.current_matches.pop_front()
//     }

//     fn len(&self) -> usize {
//         self.len
//     }
//     fn iter(&self) -> ::std::collections::hash_map::Iter<'_, Key, SmallVec<[ValBuild; 1]>> {
//         self.table.iter()
//     }
// }

use std::ops::{Deref, DerefMut};

use lattices::cc_traits::{Get, GetMut, Insert};
use lattices::map_union::{MapUnion, MapUnionSingletonMap};
use lattices::set_union::SetUnionSingletonSet;
use lattices::Merge;

type __StateHandle<Map> = hydroflow::scheduled::state::StateHandle<
    ::std::cell::RefCell<hydroflow::lattices::map_union::MapUnion<Map>>,
>;

// // Limit error propagation by bounding locally, erasing output iterator type.
// #[inline(always)]
// fn check_inputs<
//     'a,
//     Key,
//     LhsMapState,
//     RhsMapState,
//     LhsVal,
//     RhsVal,
//     LhsIter,
//     RhsIter,
//     LhsMap,
//     RhsMap,
//     LhsItem,
//     RhsItem,
// >(
//     context: &'a hydroflow::scheduled::context::Context,
//     lhs_state_handle: __StateHandle<LhsMapState>,
//     rhs_state_handle: __StateHandle<RhsMapState>,
//     lhs_iter: LhsIter,
//     rhs_iter: RhsIter,
// ) -> impl 'a + Iterator<Item = MapUnionSingletonMap<Key, SetUnionSingletonSet<(LhsVal, RhsVal)>>>
// where
//     LhsMapState:
//         'static + hydroflow::lattices::cc_traits::MapMut<Key, LhsVal, Key = Key, Item = LhsVal>,
//     RhsMapState:
//         'static + hydroflow::lattices::cc_traits::MapMut<Key, RhsVal, Key = Key, Item = RhsVal>,
//     LhsIter: Iterator<Item = MapUnion<LhsMap>>,
//     RhsIter: Iterator<Item = MapUnion<RhsMap>>,
//     MapUnion<LhsMapState>: Merge<MapUnion<LhsMap>>,
//     MapUnion<RhsMapState>: Merge<MapUnion<RhsMap>>,
//     LhsMap: Clone + IntoIterator<Item = (Key, LhsVal)>,
//     RhsMap: Clone + IntoIterator<Item = (Key, RhsVal)>,
// {
//     // for lhs_iter

//     let mut lhs_state = context.state_ref(lhs_state_handle);
//     let mut rhs_state = context.state_ref(rhs_state_handle);

//     let lhs_iter_out = {
//         lhs_iter
//             .flat_map(|lhs_map| {
//                 Merge::merge(lhs_state.borrow_mut().deref_mut(), lhs_map.clone());
//                 lhs_map.into_reveal().into_iter()
//             })
//             .flat_map(|(lhs_key, lhs_val)| {
//                 //rhs_state.borrow().deref().as_reveal_ref().get(&lhs_key).iter()
//             })
//     };

//     ::std::iter::empty()
// }

// fn lattice_join_check_inputs<'a>(
//     context: &'a
// )
