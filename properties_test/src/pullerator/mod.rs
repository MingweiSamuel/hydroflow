use crate::{Duplicates, NonMonotonic, Props, Spec};

pub trait Pullerator {
    type SpecOut: Spec;
    type PropsOut: Props;

    type ItemOut;

    fn next(&mut self) -> Option<Self::ItemOut>;
}

pub struct IteratorPullerator<Iter>
where
    Iter: IntoIterator,
{
    iter: Iter::IntoIter,
}
impl<Iter> IteratorPullerator<Iter>
where
    Iter: IntoIterator,
{
    pub fn new(iter: Iter) -> Self {
        Self {
            iter: iter.into_iter(),
        }
    }
}
impl<Iter> Pullerator for IteratorPullerator<Iter>
where
    Iter: IntoIterator,
{
    type SpecOut = IteratorSpec<Iter>;
    type PropsOut = (NonMonotonic, Duplicates);
    type ItemOut = Iter::Item;

    fn next(&mut self) -> Option<Self::ItemOut> {
        self.iter.next()
    }
}

pub struct IteratorSpec<Iter>(Iter)
where
    Iter: IntoIterator;
impl<Iter> Spec for IteratorSpec<Iter> where Iter: IntoIterator {}

pub struct ShufflePullerator<Prev>
where
    Prev: Pullerator,
{
    pub prev: Prev,
}
impl<Prev> Pullerator for ShufflePullerator<Prev>
where
    Prev: Pullerator,
{
    type SpecOut = ShuffleSpec<Prev::SpecOut>;
    type PropsOut = (NonMonotonic, <Prev::PropsOut as Props>::Duplicates);
    type ItemOut = Prev::ItemOut;

    fn next(&mut self) -> Option<Self::ItemOut> {
        self.prev.next()
    }
}

pub struct ShuffleSpec<PrevSpec>(PrevSpec)
where
    PrevSpec: Spec;
impl<PrevSpec> Spec for ShuffleSpec<PrevSpec> where PrevSpec: Spec {}

pub struct ShuffleReducePulleratorAxiom<Prev, InnerSpec>
where
    Prev: Pullerator<SpecOut = ShuffleSpec<ShuffleSpec<InnerSpec>>>,
    InnerSpec: Spec,
{
    pub prev: Prev,
}
impl<Prev, InnerSpec> Pullerator for ShuffleReducePulleratorAxiom<Prev, InnerSpec>
where
    Prev: Pullerator<SpecOut = ShuffleSpec<ShuffleSpec<InnerSpec>>>,
    InnerSpec: Spec,
{
    type SpecOut = InnerSpec;
    type PropsOut = Prev::PropsOut;
    type ItemOut = Prev::ItemOut;

    fn next(&mut self) -> Option<Self::ItemOut> {
        self.prev.next()
    }
}
