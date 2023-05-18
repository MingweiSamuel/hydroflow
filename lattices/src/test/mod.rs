//! Helper test utils to test lattice implementation correctness.

use std::fmt::Debug;

use crate::{LatticeOrd, Merge, NaiveOrd};

pub fn check_all<T: LatticeOrd + NaiveOrd + Merge<T> + Clone + Eq + Debug>(items: &[T]) {
    check_lattice_ord(items);
    check_partial_ord_properties(items);
    check_lattice_properties(items);
}

/// Check that the lattice's `PartialOrd` implementation agrees with the `NaiveOrd` partial orde
/// derived from `Merge.
pub fn check_lattice_ord<T: LatticeOrd + NaiveOrd>(items: &[T]) {
    // `NaiveOrd` is a better source of truth, as it is based on the `Merge` impl. But it
    // is inefficient. It also could be wrong if `Merge` doesn't properly return true/false
    // iff the merge changed things.
    combinations(items).for_each(|[a, b]| {
        assert_eq!(a.naive_cmp(b), a.partial_cmp(b));
        assert_eq!(b.naive_cmp(a), b.partial_cmp(a));
    });
}

/// Checks `PartialOrd`, `PartialEq`, and `Eq`'s reflexivity, symmetry, transitivity, and duality.
#[allow(clippy::eq_op)]
#[allow(clippy::double_comparisons)]
pub fn check_partial_ord_properties<T: PartialOrd + Eq>(items: &[T]) {
    use std::cmp::Ordering::*;

    // PartialEq:
    // a != b if and only if !(a == b).
    combinations(items).for_each(|[a, b]| assert_eq!(a != b, !(a == b)));

    // Eq:
    // reflexive: a == a;
    items.iter().for_each(|a| assert!(a == a));
    // symmetric: a == b implies b == a; and
    combinations(items).for_each(|[a, b]| assert_eq!(a == b, b == a));
    // transitive: a == b and b == c implies a == c.
    combinations(items).for_each(|[a, b, c]| {
        if a == b && b == c {
            assert_eq!(a == b && b == c, a == c);
        }
    });

    // PartialOrd
    combinations(items).for_each(|[a, b]| {
        // a == b if and only if partial_cmp(a, b) == Some(Equal).
        assert_eq!(a == b, a.partial_cmp(b) == Some(Equal));
        // a < b if and only if partial_cmp(a, b) == Some(Less)
        assert_eq!(a < b, a.partial_cmp(b) == Some(Less));
        // a > b if and only if partial_cmp(a, b) == Some(Greater)
        assert_eq!(a > b, a.partial_cmp(b) == Some(Greater));
        // a <= b if and only if a < b || a == b
        assert_eq!(a <= b, a < b || a == b);
        // a >= b if and only if a > b || a == b
        assert_eq!(a >= b, a > b || a == b);
        // a != b if and only if !(a == b).
        assert_eq!(a != b, !(a == b));
    });
    // transitivity: a < b and b < c implies a < c. The same must hold for both == and >.
    combinations(items).for_each(|[a, b, c]| {
        if a < b && b < c {
            assert!(a < c);
        }
        if a == b && b == c {
            assert!(a == c);
        }
        if a > b && b > c {
            assert!(a > c);
        }
    });
    // duality: a < b if and only if b > a.
    combinations(items).for_each(|[a, b]| assert_eq!(a < b, b > a));
}

/// Check lattice associativity, commutativity, and idempotence.
pub fn check_lattice_properties<T: Merge<T> + Clone + Eq + Debug>(items: &[T]) {
    // Idempotency
    // x ∧ x = x
    items
        .iter()
        .for_each(|x| assert_eq!(Merge::merge_owned(x.to_owned(), x.to_owned()), x.to_owned()));

    // Commutativity
    // x ∧ y = y ∧ x
    combinations(items).for_each(|[x, y]| {
        assert_eq!(
            Merge::merge_owned(x.to_owned(), y.to_owned()),
            Merge::merge_owned(y.to_owned(), x.to_owned())
        )
    });

    // Associativity
    // x ∧ (y ∧ z) = (x ∧ y) ∧ z
    combinations(items).for_each(|[x, y, z]| {
        assert_eq!(
            Merge::merge_owned(x.to_owned(), Merge::merge_owned(y.to_owned(), z.to_owned())),
            Merge::merge_owned(Merge::merge_owned(x.to_owned(), y.to_owned()), z.to_owned())
        )
    });
}

/// Returns an iterator of all `N`-length combinations of items in `items`.
pub fn combinations<T, const N: usize>(
    items: &[T],
) -> impl Iterator<Item = [&'_ T; N]> + ExactSizeIterator + Clone {
    struct Combinations<'a, T, const N: usize> {
        items: &'a [T],
        iters: [std::iter::Peekable<std::slice::Iter<'a, T>>; N],
    }
    impl<'a, T, const N: usize> Iterator for Combinations<'a, T, N> {
        type Item = [&'a T; N];

        fn next(&mut self) -> Option<Self::Item> {
            if self.items.is_empty() {
                return None;
            }

            let mut go_next = true;
            let combo = std::array::from_fn::<_, N, _>(|i| {
                let iter = &mut self.iters[i];
                let &item = iter.peek().unwrap();
                if go_next {
                    iter.next();
                    if iter.peek().is_none() {
                        *iter = self.items.iter().peekable();
                    } else {
                        go_next = false;
                    }
                }
                item
            });
            if go_next {
                self.items = &[];
            };
            Some(combo)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            if self.items.is_empty() {
                return (0, Some(0));
            }
            let mut pow = 1;
            let mut passed = 0;
            for iter in self.iters.iter() {
                passed += pow * (self.items.len() - iter.len());
                pow *= self.items.len();
            }
            let size = pow - passed;
            (size, Some(size))
        }
    }
    impl<'a, T, const N: usize> ExactSizeIterator for Combinations<'a, T, N> {}
    impl<'a, T, const N: usize> Clone for Combinations<'a, T, N> {
        fn clone(&self) -> Self {
            Self {
                items: self.items.clone(),
                iters: self.iters.clone(),
            }
        }
    }
    let iters = std::array::from_fn::<_, N, _>(|_| items.iter().peekable());
    Combinations { items, iters }
}

#[test]
fn test_combinations() {
    let mut iter = combinations(&[1, 2, 3]).enumerate();
    println!("{}", iter.len());
    while let Some((i, [a, b, c])) = iter.next() {
        println!("{}: {:?} ({})", i + 1, [a, b, c], iter.len());
    }
}

#[test]
fn test_combinations_empty() {
    let mut iter = combinations::<_, 4>(&[] as &[usize]);
    assert_eq!(0, iter.len());
    assert_eq!(None, iter.next());
}
