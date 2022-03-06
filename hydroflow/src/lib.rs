#![feature(generic_associated_types)]

pub trait MyTrait {
    type Keys<'s>
    where
        Self: 's;
    fn keys(&self) -> Self::Keys<'_>;
}

impl MyTrait for () {
    type Keys<'s> = &'s ();
    fn keys(&self) -> Self::Keys<'_> {
        &()
    }
}
