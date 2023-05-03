//! Simple singleton or array collection with [`cc_traits`] implementations.

use std::array::IntoIter;
use std::borrow::Borrow;
use std::hash::Hash;

use crate::cc_traits::{
    Collection, CollectionMut, CollectionRef, Get, GetKeyValue, GetMut, Iter as CcIter, Keyed,
    KeyedRef, Len, MapIter,
};

/// A singleton wrapper which implements `Collection`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Single<T>(pub T);
impl<T> IntoIterator for Single<T> {
    type Item = T;
    type IntoIter = std::iter::Once<T>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.0)
    }
}
impl<T> From<T> for Single<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

/// A wrapper around an item, representing a singleton set.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SingletonSet<T>(pub T);
impl<T> IntoIterator for SingletonSet<T> {
    type Item = T;
    type IntoIter = std::iter::Once<T>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self.0)
    }
}
impl<T> From<T> for SingletonSet<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
impl<T> Collection for SingletonSet<T> {
    type Item = T;
}
impl<T> Len for SingletonSet<T> {
    fn len(&self) -> usize {
        1
    }
}
impl<T> CollectionRef for SingletonSet<T> {
    type ItemRef<'a> = &'a T
    where
        Self: 'a;

    cc_traits::covariant_item_ref!();
}
impl<'a, Q, T> Get<&'a Q> for SingletonSet<T>
where
    T: Borrow<Q>,
    Q: Eq + ?Sized,
{
    fn get(&self, key: &'a Q) -> Option<Self::ItemRef<'_>> {
        (key == self.0.borrow()).then_some(&self.0)
    }
}
impl<T> CollectionMut for SingletonSet<T> {
    type ItemMut<'a> = &'a mut T
    where
        Self: 'a;

    cc_traits::covariant_item_mut!();
}
impl<'a, Q, T> GetMut<&'a Q> for SingletonSet<T>
where
    T: Borrow<Q>,
    Q: Eq + ?Sized,
{
    fn get_mut(&mut self, key: &'a Q) -> Option<Self::ItemMut<'_>> {
        (key == self.0.borrow()).then_some(&mut self.0)
    }
}
impl<T> CcIter for SingletonSet<T> {
    type Iter<'a> = std::iter::Once<&'a T>
	where
		Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        std::iter::once(&self.0)
    }
}

/// A key-value entry wrapper representing a singleton map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SingletonMap<K, V>(pub K, pub V);
impl<K, V> IntoIterator for SingletonMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::iter::Once<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once((self.0, self.1))
    }
}
impl<K, V> From<(K, V)> for SingletonMap<K, V> {
    fn from((k, v): (K, V)) -> Self {
        Self(k, v)
    }
}
impl<K, V> Collection for SingletonMap<K, V> {
    type Item = V;
}
impl<K, V> Len for SingletonMap<K, V> {
    fn len(&self) -> usize {
        1
    }
}
impl<K, V> CollectionRef for SingletonMap<K, V> {
    type ItemRef<'a> = &'a V
    where
        Self: 'a;

    cc_traits::covariant_item_ref!();
}
impl<'a, Q, K, V> Get<&'a Q> for SingletonMap<K, V>
where
    K: Borrow<Q>,
    Q: Eq + ?Sized,
{
    fn get(&self, key: &'a Q) -> Option<Self::ItemRef<'_>> {
        (key == self.0.borrow()).then_some(&self.1)
    }
}
impl<'a, Q, K, V> GetKeyValue<&'a Q> for SingletonMap<K, V>
where
    K: Borrow<Q>,
    Q: Eq + ?Sized,
{
    fn get_key_value(&self, key: &'a Q) -> Option<(Self::KeyRef<'_>, Self::ItemRef<'_>)> {
        (key == self.0.borrow()).then_some((&self.0, &self.1))
    }
}
impl<K, V> Keyed for SingletonMap<K, V> {
    type Key = K;
}
impl<K, V> KeyedRef for SingletonMap<K, V> {
    type KeyRef<'a> = &'a K
	where
		Self: 'a;

    cc_traits::covariant_key_ref!();
}
// impl<K, V> SimpleKeyedRef for SingletonMap<K, V> {
//     cc_traits::simple_keyed_ref!();
// }
impl<K, V> MapIter for SingletonMap<K, V> {
    type Iter<'a> = std::iter::Once<(&'a K, &'a V)>
	where
		Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        std::iter::once((&self.0, &self.1))
    }
}

/// A fixed-sized array wrapper which implements `Collection`.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array<T, const N: usize>(pub [T; N]);
impl<T, const N: usize> IntoIterator for Array<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}
impl<T, const N: usize> From<[T; N]> for Array<T, N> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}
impl<T, const N: usize> Collection for Array<T, N> {
    type Item = T;
}
impl<T, const N: usize> Len for Array<T, N> {
    fn len(&self) -> usize {
        N
    }
}
impl<T, const N: usize> CollectionRef for Array<T, N> {
    type ItemRef<'a> = &'a T
    where
        Self: 'a;

    cc_traits::covariant_item_ref!();
}
impl<T, const N: usize> Get<usize> for Array<T, N> {
    fn get(&self, key: usize) -> Option<Self::ItemRef<'_>> {
        self.0.get(key)
    }
}
impl<T, const N: usize> CollectionMut for Array<T, N> {
    type ItemMut<'a> = &'a mut T
    where
        Self: 'a;

    cc_traits::covariant_item_mut!();
}
impl<T, const N: usize> GetMut<usize> for Array<T, N> {
    fn get_mut(&mut self, key: usize) -> Option<Self::ItemMut<'_>> {
        self.0.get_mut(key)
    }
}
impl<T, const N: usize> CcIter for Array<T, N> {
    type Iter<'a> = std::slice::Iter<'a, T>
	where
		Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter()
    }
}

/// An array wrapper representing a fixed-size set (moduoo duplicate items).
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArraySet<T, const N: usize>(pub [T; N]);
impl<T, const N: usize> IntoIterator for ArraySet<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.0)
    }
}
impl<T, const N: usize> From<[T; N]> for ArraySet<T, N> {
    fn from(value: [T; N]) -> Self {
        Self(value)
    }
}
impl<T, const N: usize> Collection for ArraySet<T, N> {
    type Item = T;
}
impl<T, const N: usize> Len for ArraySet<T, N> {
    fn len(&self) -> usize {
        N
    }
}
impl<T, const N: usize> CollectionRef for ArraySet<T, N> {
    type ItemRef<'a> = &'a T
    where
        Self: 'a;

    cc_traits::covariant_item_ref!();
}
impl<'a, Q, T, const N: usize> Get<&'a Q> for ArraySet<T, N>
where
    T: Borrow<Q>,
    Q: Eq + ?Sized,
{
    fn get(&self, key: &'a Q) -> Option<Self::ItemRef<'_>> {
        self.0
            .iter()
            .position(|item| key == item.borrow())
            .map(|i| &self.0[i])
    }
}
impl<T, const N: usize> CollectionMut for ArraySet<T, N> {
    type ItemMut<'a> = &'a mut T
    where
        Self: 'a;

    cc_traits::covariant_item_mut!();
}
impl<'a, Q, T, const N: usize> GetMut<&'a Q> for ArraySet<T, N>
where
    T: Borrow<Q>,
    Q: Eq + ?Sized,
{
    fn get_mut(&mut self, key: &'a Q) -> Option<Self::ItemMut<'_>> {
        self.0
            .iter()
            .position(|item| key == item.borrow())
            .map(|i| &mut self.0[i])
    }
}
impl<T, const N: usize> CcIter for ArraySet<T, N> {
    type Iter<'a> = std::slice::Iter<'a, T>
	where
		Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.0.iter()
    }
}

/// A boolean-masked fixed-size array wrapper which implements `Collection`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaskedArray<T, const N: usize> {
    /// The boolean mask.
    pub mask: [bool; N],
    /// The collection items.
    pub vals: [T; N],
}
impl<T, const N: usize> IntoIterator for MaskedArray<T, N> {
    type Item = T;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(self.mask)
            .zip(IntoIterator::into_iter(self.vals))
            .filter(|(mask, _)| *mask)
            .map(|(_, val)| val)
    }
}
