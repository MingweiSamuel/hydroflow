#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]


use crate::pullerator::{IteratorPullerator, ShufflePullerator, ShuffleReducePulleratorAxiom, Pullerator, IteratorSpec};

pub mod pullerator;
pub mod pusherator;

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
