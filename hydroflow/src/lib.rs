#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

use std::collections::HashSet;
use std::hash::Hash;

pub trait Collection<K, V> {
    type Keys<'s>: Iterator<Item = &'s K>
    where
        K: 's,
        Self: 's;
    fn keys(&self) -> Self::Keys<'_>;
}

impl<K: 'static + Eq + Hash> Collection<K, ()> for HashSet<K> {
    type Keys<'s> = std::collections::hash_set::Iter<'s, K>;
    fn keys(&self) -> Self::Keys<'_> {
        self.iter()
    }
}
