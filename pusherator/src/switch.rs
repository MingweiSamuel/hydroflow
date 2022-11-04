use std::marker::PhantomData;

use super::Pusherator;

use type_list::Variadic;

pub trait PusheratorList<Item>: Variadic {
    fn give(&mut self, idx: usize, item: Item);
}
impl<Item> PusheratorList<Item> for () {
    #[inline]
    fn give(&mut self, idx: usize, _item: Item) {
        panic!("`idx` greater than length of `PusheratorList`: {}", idx);
    }
}
impl<Item, Push, Rest> PusheratorList<Item> for (Push, Rest)
where
    Push: Pusherator<Item = Item>,
    Rest: PusheratorList<Item>,
{
    #[inline]
    fn give(&mut self, idx: usize, item: Item) {
        if 0 == idx {
            self.0.give(item);
        } else {
            self.1.give(idx - 1, item);
        }
    }
}

pub struct Switch<NextList, Item> {
    next_list: NextList,
    _phantom: PhantomData<fn(Item)>,
}
impl<NextList, Item> Pusherator for Switch<NextList, Item>
where
    NextList: PusheratorList<Item>,
{
    type Item = (usize, Item);
    fn give(&mut self, item: Self::Item) {
        self.next_list.give(item.0, item.1);
    }
}
impl<NextList, Item> Switch<NextList, Item> {
    pub fn new(next_list: NextList) -> Self {
        Self {
            next_list,
            _phantom: PhantomData,
        }
    }
}
