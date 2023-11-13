use variadics::*;

pub trait Boxed: Variadic {
    type Boxed;
    fn boxed(self) -> Self::Boxed;
}
impl<Item, Rest> Boxed for (Item, Rest)
where
    Rest: Boxed,
{
    type Boxed = Var!(Box<Item>, ...Rest::Boxed);
    fn boxed(self) -> Self::Boxed {
        let (item, rest) = self;
        var!(Box::new(item), ...rest.boxed())
    }
}
impl Boxed for () {
    type Boxed = ();
    fn boxed(self) -> Self::Boxed {}
}

pub trait Refed: Variadic {
    type Refed<'a>
    where
        Self: 'a;
    fn refed(&self) -> Self::Refed<'_>;
}
impl<Item, Rest> Refed for (Item, Rest)
where
    Rest: Refed,
{
    type Refed<'a> = (&'a Item, Rest::Refed<'a>)
    where
        Self: 'a;
    fn refed(&self) -> Self::Refed<'_> {
        let (item, rest) = self;
        (item, rest.refed())
    }
}
impl Refed for () {
    type Refed<'a> = ();
    fn refed(&self) -> Self::Refed<'_> {}
}
