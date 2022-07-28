#![feature(type_alias_impl_trait)]

pub trait Pullerator {
    type SpecOut: Spec;
    type PropsOut: Props;

    type ItemOut;

    fn next(&mut self) -> Option<Self::ItemOut>;
}
pub trait Spec {}

pub trait Props {
    type Monotonicity: PropMonotonicity;
    type Duplicates: PropDuplicates;
}
impl<Monotonicity, Duplicates> Props for (Monotonicity, Duplicates)
where
    Monotonicity: PropMonotonicity,
    Duplicates: PropDuplicates,
{
    type Monotonicity = Monotonicity;
    type Duplicates = Duplicates;
}

pub trait PropMonotonicity {}
pub struct NonMonotonic;
impl PropMonotonicity for NonMonotonic {}
pub struct Monotonic;
impl PropMonotonicity for Monotonic {}
pub struct Consecutive;
impl PropMonotonicity for Consecutive {}

pub trait PropDuplicates {}
pub struct Duplicates;
impl PropDuplicates for Duplicates {}
pub struct NoDuplicates;
impl PropDuplicates for NoDuplicates {}

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
    prev: Prev,
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
    prev: Prev,
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

// #[test]
fn test_shuffle_reduce() {
    let x = [1, 2, 3, 4];

    // type X = impl Pullerator<SpecOut = IteratorSpec<[usize; 4]>>;

    let prev = IteratorPullerator::new(x);
    let prev = ShufflePullerator { prev };
    let prev = ShufflePullerator { prev };
    let prev = ShuffleReducePulleratorAxiom { prev };

    fn assert_impl<X>(_: X)
    where
        X: Pullerator<SpecOut = IteratorSpec<[usize; 4]>>,
    {
    }
    assert_impl(prev);
}
