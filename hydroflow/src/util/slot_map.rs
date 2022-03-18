pub trait Key: From<KeyData> + Into<usize> {}

pub struct KeyData(usize);

#[macro_export]
macro_rules! new_key_type {
    ( $(#[$outer:meta])* $vis:vis struct $name:ident; $($rest:tt)* ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
        #[repr(transparent)]
        $vis struct $name(usize);

        impl Into<usize> for $name {
            fn into(self) -> Self {
                self.o
            }
        }
        impl $crate::util::slot_map::Key for $name {}

        $crate::util::slot_map::new_key_type!($($rest)*);
    };
    () => {};
}
pub use new_key_type;

pub struct SlotMap<K, T>
where
    K: Key,
{
    items: Vec<T>,
}
impl<K, T> Default for SlotMap<K, T>
where
    K: Key,
{
    fn default() -> Self {
        Self {
            items: Default::default(),
        }
    }
}
impl<K, T> SlotMap<K, T>
where
    K: Key,
{
    pub fn new() -> Self {
        Default::default()
    }
    pub fn insert_with_key(func: impl FnOnce(K) -> T) -> K {
        let key = self.0.len();
        let val = (func)(key);
        val.push(func);
    }
}
