//! Module containing the [`UnionFind`] lattice and aliases for different datastructures.

use std::cell::Cell;
use std::cmp::Ordering::{self, *};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;

use crate::cc_traits::{
    GetMut, Iter, Keyed, Len, Map, MapIter, MapMut, SimpleCollectionRef, SimpleKeyedRef,
};
use crate::collections::{ArrayMap, OptionMap, SingletonMap, VecMap};
use crate::{Atomize, IsBot, IsTop, LatticeFrom, LatticeOrd, Max, Merge, Min, NaiveLatticeOrd};

// TODO(mingwei): handling malformed trees - parents must be Ord smaller than children.

/// Union-find lattice.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionFind<Map>(Map);
impl<Map> UnionFind<Map> {
    /// Create a new `UnionFind` from a `Map`.
    pub fn new(val: Map) -> Self {
        Self(val)
    }

    /// Create a new `UnionFind` from an `Into<Map>`.
    pub fn new_from(val: impl Into<Map>) -> Self {
        Self::new(val.into())
    }

    /// Reveal the inner value as a shared reference.
    pub fn as_reveal_ref(&self) -> &Map {
        &self.0
    }

    /// Reveal the inner value as an exclusive reference.
    pub fn as_reveal_mut(&mut self) -> &mut Map {
        &mut self.0
    }

    /// Gets the inner by value, consuming self.
    pub fn into_reveal(self) -> Map {
        self.0
    }
}

impl<MapSelf, T> UnionFind<MapSelf>
where
    MapSelf: MapMut<T, Cell<T>, Key = T, Item = Cell<T>>,
    T: Copy + Ord,
{
    /// Union the sets containg `a` and `b`.
    ///
    /// Returns true if the sets changed, false if `a` and `b` were already in the same set. Once
    /// this returns false it will always return false for the same `a` and `b`, therefore it
    /// returns a `Min<bool>` lattice.
    pub fn union(&mut self, a: T, b: T) -> Min<bool> {
        let mut a_root = self.find(a).into_reveal();
        let mut b_root = self.find(b).into_reveal();
        if a_root == b_root {
            return Min::new(false);
        }
        if b_root < a_root {
            (a_root, b_root) = (b_root, a_root);
        }
        self.0.insert(b_root, Cell::new(a_root));
        Min::new(true)
    }
}

impl<MapSelf, T> UnionFind<MapSelf>
where
    MapSelf: Map<T, Cell<T>, Key = T, Item = Cell<T>>,
    T: Copy + Ord,
{
    /// Returns if `a` and `b` are in the same set.
    ///
    /// This method is monotonic: once this returns true it will always return true for the same
    /// `a` and `b`, therefore it returns a `Max<bool>` lattice.
    pub fn same(&self, a: T, b: T) -> Max<bool> {
        Max::new(self.find(a) == self.find(b))
    }

    /// Finds the representative root node for `item`.
    ///
    /// Ties are broken by choosing the minimum element, therefore this returns a `Min<T>` lattice.
    pub fn find(&self, mut item: T) -> Min<T> {
        let mut root = item;
        while let Some(parent) = self.0.get(&root) {
            if parent.get() == root {
                break;
            }
            root = parent.get();
        }
        while item != root {
            item = self.0.get(&item).unwrap().replace(root);
        }
        Min::new(item)
    }

    // pub fn find_ref(&self, mut item: T) -> Min<T> {
    //     while let Some(&parent) = self.0.get(&item).as_deref() {
    //         if parent == item {
    //             break;
    //         }
    //         item = parent;
    //     }
    //     Min::new(item)
    // }
}

impl<MapSelf, MapOther, T> Merge<UnionFind<MapOther>> for UnionFind<MapSelf>
where
    MapSelf: MapMut<T, Cell<T>, Key = T, Item = Cell<T>>,
    MapOther: IntoIterator<Item = (T, Cell<T>)>,

    T: Copy + Ord,
{
    fn merge(&mut self, other: UnionFind<MapOther>) -> bool {
        let mut changed = false;
        for (item, parent) in other.0.into_iter() {
            // Do not short circuit.
            changed |= self.union(item, parent.get()).into_reveal();
        }
        changed
    }
}

impl<MapSelf, MapOther, T> LatticeFrom<UnionFind<MapOther>> for UnionFind<MapSelf>
where
    MapSelf: Keyed<Key = T, Item = Cell<T>> + FromIterator<(T, Cell<T>)>,
    MapOther: IntoIterator<Item = (T, Cell<T>)>,
    T: Copy + Ord,
{
    fn lattice_from(other: UnionFind<MapOther>) -> Self {
        Self(other.0.into_iter().collect())
    }
}

// impl<MapSelf, MapOther, T> PartialOrd<UnionFind<MapOther>> for UnionFind<MapSelf>
// where
//     MapSelf: MapMut<T, Cell<T>, Key = T, Item = Cell<T>> + MapIter + SimpleKeyedRef,
//     MapOther: MapMut<T, Cell<T>, Key = T, Item = Cell<T>> + MapIter + SimpleKeyedRef,
//     T: Copy + Ord,
//     // TODO(mingwei).
//     Self: Clone + Merge<UnionFind<MapOther>> + Merge<UnionFind<MapOther>>,
//     UnionFind<MapOther>: Clone + Merge<Self>,
//     MapSelf: IntoIterator<Item = (T, Cell<T>)>,
// {
//     fn partial_cmp(&self, other: &UnionFind<MapOther>) -> Option<Ordering> {
//         // TODO(mingwei); (fix trait bounds)
//         <Self as NaiveLatticeOrd>::naive_cmp(&self, other)
//     }
// }
impl<Map> PartialOrd<Self> for UnionFind<Map>
where
    Self: Clone + PartialEq + Merge<Self>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        <Self as NaiveLatticeOrd>::naive_cmp(&self, other)
    }
}
impl<MapSelf, MapOther> LatticeOrd<UnionFind<MapOther>> for UnionFind<MapSelf> where
    Self: PartialOrd<UnionFind<MapOther>>
{
}

impl<MapSelf, MapOther, T> PartialEq<UnionFind<MapOther>> for UnionFind<MapSelf>
where
    MapSelf: MapMut<T, Cell<T>, Key = T, Item = Cell<T>> + MapIter + SimpleKeyedRef,
    MapOther: MapMut<T, Cell<T>, Key = T, Item = Cell<T>> + MapIter + SimpleKeyedRef,
    T: Copy + Ord,
{
    fn eq(&self, other: &UnionFind<MapOther>) -> bool {
        for (item, parent) in self.0.iter() {
            if !other.same(*item, parent.get()).into_reveal() {
                return false;
            }
        }
        for (item, parent) in other.0.iter() {
            if !self.same(*item, parent.get()).into_reveal() {
                return false;
            }
        }
        true
    }
}
impl<MapSelf> Eq for UnionFind<MapSelf> where Self: PartialEq {}

impl<Map, T> IsBot for UnionFind<Map>
where
    Map: MapIter<Key = T, Item = Cell<T>>,
    T: Copy + Ord,
{
    fn is_bot(&self) -> bool {
        self.0.iter().all(|(a, b)| *a == b.get())
    }
}

impl<Map> IsTop for UnionFind<Map> {
    fn is_top(&self) -> bool {
        false
    }
}

// impl<Map, K, Val> Atomize for UnionFind<Map>
// where
//     Map: 'static
//         + Len
//         + IntoIterator<Item = (K, Val)>
//         + Keyed<Key = K, Item = Val>
//         + Extend<(K, Val)>
//         + for<'a> GetMut<&'a K, Item = Val>,
//     K: 'static + Clone,
//     Val: 'static + Atomize + LatticeFrom<<Val as Atomize>::Atom>,
// {
//     type Atom = UnionFindOptionMap<K, Val::Atom>;

//     // TODO: use impl trait.
//     type AtomIter = Box<dyn Iterator<Item = Self::Atom>>;

//     fn atomize(self) -> Self::AtomIter {
//         Box::new(self.0.into_iter().flat_map(|(k, val)| {
//             val.atomize()
//                 .map(move |v| UnionFindOptionMap::new_from((k.clone(), v)))
//         }))
//     }
// }

/// [`std::collections::HashMap`]-backed [`UnionFind`] lattice.
pub type UnionFindHashMap<T> = UnionFind<HashMap<T, Cell<T>>>;

/// [`std::collections::BTreeMap`]-backed [`UnionFind`] lattice.
pub type UnionFindBTreeMap<T> = UnionFind<BTreeMap<T, Cell<T>>>;

/// [`Vec`]-backed [`UnionFind`] lattice.
pub type UnionFindVec<T> = UnionFind<VecMap<T, Cell<T>>>;

/// Array-backed [`UnionFind`] lattice.
pub type UnionFindArrayMap<T, const N: usize> = UnionFind<ArrayMap<T, Cell<T>, N>>;

/// [`crate::collections::SingletonMap`]-backed [`UnionFind`] lattice.
pub type UnionFindSingletonMap<T> = UnionFind<SingletonMap<T, Cell<T>>>;

/// [`Option`]-backed [`UnionFind`] lattice.
pub type UnionFindOptionMap<T> = UnionFind<OptionMap<T, Cell<T>>>;

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use super::*;
    use crate::collections::SingletonMap;
    use crate::set_union::SetUnionHashSet;
    use crate::test::{
        cartesian_power, check_all, check_atomize_each, check_lattice_is_bot, check_lattice_is_top,
        check_lattice_ord, check_lattice_properties, check_partial_ord_properties,
    };

    #[test]
    fn test_basic() {
        let mut my_map_a = <UnionFindHashMap<&str>>::default();
        let my_map_b =
            <UnionFindSingletonMap<&str>>::new(SingletonMap("hello", Cell::new("world")));
        let my_map_c = UnionFindSingletonMap::new_from(("hello", Cell::new("goodbye")));

        assert!(!my_map_a.same("hello", "world").into_reveal());
        assert!(!my_map_a.same("hello", "goodbye").into_reveal());
        assert!(!my_map_a.same("world", "goodbye").into_reveal());
        assert_eq!("foo", my_map_a.find("foo").into_reveal());

        my_map_a.merge(my_map_b);

        assert!(my_map_a.same("hello", "world").into_reveal());
        assert!(!my_map_a.same("hello", "goodbye").into_reveal());
        assert!(!my_map_a.same("world", "goodbye").into_reveal());
        assert_eq!("foo", my_map_a.find("foo").into_reveal());

        my_map_a.merge(my_map_c);

        assert!(my_map_a.same("hello", "world").into_reveal());
        assert!(my_map_a.same("hello", "goodbye").into_reveal());
        assert!(my_map_a.same("world", "goodbye").into_reveal());
        assert_eq!("foo", my_map_a.find("foo").into_reveal());
    }

    #[test]
    fn consistency() {
        let items = &[
            <UnionFindHashMap<char>>::default(),
            <UnionFindHashMap<_>>::new_from([('a', Cell::new('a'))]),
            <UnionFindHashMap<_>>::new_from([('a', Cell::new('a')), ('b', Cell::new('a'))]),
            <UnionFindHashMap<_>>::new_from([('b', Cell::new('a'))]),
            <UnionFindHashMap<_>>::new_from([('b', Cell::new('a')), ('c', Cell::new('b'))]),
            <UnionFindHashMap<_>>::new_from([('b', Cell::new('a')), ('c', Cell::new('b'))]),
            <UnionFindHashMap<_>>::new_from([('d', Cell::new('b'))]),
        ];

        check_all(items);
    }

    // #[test]
    // fn consistency_atomize() {
    //     let mut test_vec = Vec::new();

    //     // Size 0.
    //     test_vec.push(UnionFindHashMap::default());
    //     // Size 1.
    //     for key in [0, 1] {
    //         for value in [vec![], vec![0], vec![1], vec![0, 1]] {
    //             test_vec.push(UnionFindHashMap::new(HashMap::from_iter([(
    //                 key,
    //                 SetUnionHashSet::new(HashSet::from_iter(value)),
    //             )])));
    //         }
    //     }
    //     // Size 2.
    //     for [val_a, val_b] in cartesian_power(&[vec![], vec![0], vec![1], vec![0, 1]]) {
    //         test_vec.push(UnionFindHashMap::new(HashMap::from_iter([
    //             (0, SetUnionHashSet::new(HashSet::from_iter(val_a.clone()))),
    //             (1, SetUnionHashSet::new(HashSet::from_iter(val_b.clone()))),
    //         ])));
    //     }

    //     check_all(&test_vec);
    //     check_atomize_each(&test_vec);
    // }

    // /// Check that a key with a value of bottom is the same as an empty map, etc.
    // #[test]
    // fn test_collapes_bot() {
    //     let map_empty = <UnionFindHashMap<&str, SetUnionHashSet<u64>>>::default();
    //     let map_a_bot = <UnionFindSingletonMap<&str, SetUnionHashSet<u64>>>::new(SingletonMap(
    //         "a",
    //         Default::default(),
    //     ));
    //     let map_b_bot = <UnionFindSingletonMap<&str, SetUnionHashSet<u64>>>::new(SingletonMap(
    //         "b",
    //         Default::default(),
    //     ));

    //     assert_eq!(map_empty, map_a_bot);
    //     assert_eq!(map_empty, map_b_bot);
    //     assert_eq!(map_a_bot, map_b_bot);
    // }
}
