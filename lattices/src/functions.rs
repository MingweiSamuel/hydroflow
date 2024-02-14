//! Module containing special functions (maps) for lattices.

use std::convert::Infallible;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use cc_traits::{GetKeyValue, Insert, Iter, MapIter, SimpleCollectionRef, SimpleKeyedRef};

use crate::map_union::{MapUnion, MapUnionHashMap};
use crate::set_union::SetUnion;
use crate::test::cartesian_power;
use crate::{Merge, Pair};

// TODO: docs. Semilattice binary homomorphism.
pub trait LatticeBimorphism<LatA, LatB> {
    type Output;
    fn call(&mut self, lat_a: LatA, lat_b: LatB) -> Self::Output;
}

pub fn wrap_closure_lattice_bimorphism<LatA, LatB, LatOut, F>(
    func: F,
) -> impl LatticeBimorphism<LatA, LatB, Output = LatOut>
where
    F: FnMut(LatA, LatB) -> LatOut,
    LatA: Merge<LatA>,
    LatB: Merge<LatB>,
    LatOut: Merge<LatOut>,
{
    struct FnLatticeBimorphism<F>(F);
    impl<Func, LatA, LatB, LatOut> LatticeBimorphism<LatA, LatB> for FnLatticeBimorphism<Func>
    where
        Func: FnMut(LatA, LatB) -> LatOut,
        LatA: Merge<LatA>,
        LatB: Merge<LatB>,
        LatOut: Merge<LatOut>,
    {
        type Output = LatOut;

        fn call(&mut self, lat_a: LatA, lat_b: LatB) -> Self::Output {
            (self.0)(lat_a, lat_b)
        }
    }
    FnLatticeBimorphism(func)
}

// pub fn key_lattice_bimorphism<LatA, LatB, LatOut, Func>(lattice_bimorphism: Func) {
//     struct KeyLatticeBimorphism<Func>(Func);
// }
struct KeyLatticeBimorphism<Func>(Func);
impl<MapA, MapB, ValFunc> LatticeBimorphism<MapUnion<MapA>, MapUnion<MapB>>
    for KeyLatticeBimorphism<ValFunc>
where
    ValFunc: LatticeBimorphism<MapA::Item, MapB::Item>,
    MapA: MapIter + SimpleKeyedRef + SimpleCollectionRef,
    MapB: for<'a> GetKeyValue<&'a MapA::Key, Key = MapA::Key> + SimpleCollectionRef,
    MapA::Key: Clone + Eq + Hash,
    MapA::Item: Clone,
    MapB::Item: Clone,
{
    type Output = MapUnionHashMap<MapA::Key, ValFunc::Output>;

    fn call(&mut self, lat_a: MapUnion<MapA>, lat_b: MapUnion<MapB>) -> Self::Output {
        let mut output = MapUnionHashMap::default();
        for (key, val_a) in lat_a.as_reveal_ref().iter() {
            let key = <MapA as SimpleKeyedRef>::into_ref(key);
            let Some((_key, val_b)) = lat_b.as_reveal_ref().get_key_value(key) else {
                continue;
            };
            let val_a = <MapA as SimpleCollectionRef>::into_ref(val_a).clone();
            let val_b = <MapB as SimpleCollectionRef>::into_ref(val_b).clone();

            let val_out = LatticeBimorphism::call(&mut self.0, val_a, val_b);
            output.as_reveal_mut().insert(key.clone(), val_out);
        }
        output
    }
}

pub fn check_lattice_bimorphism<LatA, LatB, Func>(
    mut func: Func,
    items_a: &[LatA],
    items_b: &[LatB],
) where
    Func: LatticeBimorphism<LatA, LatB>,
    LatA: Merge<LatA> + Clone + Eq + Debug,
    LatB: Merge<LatB> + Clone + Eq + Debug,
    Func::Output: Merge<Func::Output> + Clone + Eq + Debug,
{
    // Morphism LHS, fixed RHS:
    for b in items_b {
        for [a, da] in cartesian_power(items_a) {
            assert_eq!(
                func.call(Merge::merge_owned(a.clone(), da.clone()), b.clone()),
                Merge::merge_owned(
                    func.call(a.clone(), b.clone()),
                    func.call(da.clone(), b.clone())
                ),
                "Left arg not a morphism: `f(a ⊔ da, b) != f(a, b) ⊔ f(da, b)`
                \n`a = {:?}`, da = {:?}, b = {:?}",
                a,
                da,
                b,
            );
        }
    }
    // Fixed LHS, morphism RHS:
    for a in items_a {
        for [b, db] in cartesian_power(items_b) {
            assert_eq!(
                func.call(a.clone(), Merge::merge_owned(b.clone(), db.clone())),
                Merge::merge_owned(
                    func.call(a.clone(), b.clone()),
                    func.call(a.clone(), db.clone())
                ),
                "Right arg not a morphism: `f(a, b ⊔ db) != f(a, b) ⊔ f(a, db)`
                \n`a = {:?}`, b = {:?}, db = {:?}",
                a,
                b,
                db,
            );
        }
    }
}

pub struct CartesianProductFn<SetOut> {
    _phantom: PhantomData<fn() -> SetOut>,
}
impl<SetOut> Default for CartesianProductFn<SetOut> {
    fn default() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
}

impl<SetA, SetB, SetOut> LatticeBimorphism<SetUnion<SetA>, SetUnion<SetB>>
    for CartesianProductFn<SetOut>
where
    SetA: IntoIterator,
    SetB: Iter + SimpleCollectionRef,
    SetA::Item: Clone,
    SetB::Item: Clone,
    SetOut: FromIterator<(SetA::Item, SetB::Item)>,
{
    type Output = SetUnion<SetOut>;

    fn call(&mut self, lat_a: SetUnion<SetA>, lat_b: SetUnion<SetB>) -> Self::Output {
        let set_a = lat_a.into_reveal();
        let set_b = lat_b.into_reveal();
        let set_out = set_a
            .into_iter()
            .flat_map(|a_item| {
                set_b
                    .iter()
                    .map(<SetB as SimpleCollectionRef>::into_ref)
                    .cloned()
                    .map(move |b_item| (a_item.clone(), b_item))
            })
            .collect::<SetOut>();
        SetUnion::new(set_out)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::{
        check_lattice_bimorphism, wrap_closure_lattice_bimorphism, CartesianProductFn,
        KeyLatticeBimorphism,
    };
    use crate::map_union::{MapUnionHashMap, MapUnionSingletonMap};
    use crate::set_union::{SetUnionHashSet, SetUnionSingletonSet};
    use crate::Pair;

    #[test]
    fn test_lattice_bimorphism_pair() {
        let items = &[
            SetUnionHashSet::new_from([]),
            SetUnionHashSet::new_from([0]),
            SetUnionHashSet::new_from([1]),
            SetUnionHashSet::new_from([0, 1]),
        ];
        let func = wrap_closure_lattice_bimorphism(|l, r| Pair::new(l, r));
        check_lattice_bimorphism(func, items, items);
    }

    #[test]
    fn test_lattice_bimorphism_cartesian() {
        let items = &[
            SetUnionHashSet::new_from([]),
            SetUnionHashSet::new_from([0]),
            SetUnionHashSet::new_from([1]),
            SetUnionHashSet::new_from([0, 1]),
        ];
        let func = wrap_closure_lattice_bimorphism(
            |l: SetUnionHashSet<usize>, r: SetUnionHashSet<usize>| {
                let l = l.into_reveal();
                let r = r.into_reveal();
                let out = l
                    .into_iter()
                    .flat_map(|l_item| r.iter().cloned().map(move |r_item| (l_item, r_item)))
                    .collect::<HashSet<_>>();
                SetUnionHashSet::new(out)
            },
        );
        check_lattice_bimorphism(func, items, items);
    }

    #[test]
    fn test_join() {
        let items_a = &[
            MapUnionHashMap::new_from([("foo", SetUnionHashSet::new_from(["bar"]))]),
            MapUnionHashMap::new_from([("foo", SetUnionHashSet::new_from(["baz"]))]),
            MapUnionHashMap::new_from([("hello", SetUnionHashSet::new_from(["world"]))]),
        ];
        let items_b = &[
            MapUnionHashMap::new_from([("foo", SetUnionHashSet::new_from(["bang"]))]),
            MapUnionHashMap::new_from([(
                "hello",
                SetUnionHashSet::new_from(["goodbye", "farewell"]),
            )]),
        ];
        // let cartesian_product = wrap_closure_lattice_bimorphism(
        //     |l: SetUnionHashSet<&'static str>, r: SetUnionHashSet<&'static str>| {
        //         let l = l.into_reveal();
        //         let r = r.into_reveal();
        //         let out = l
        //             .into_iter()
        //             .flat_map(|l_item| r.iter().cloned().map(move |r_item| (l_item, r_item)))
        //             .collect::<HashSet<_>>();
        //         SetUnionHashSet::new(out)
        //     },
        // );
        let func = KeyLatticeBimorphism(CartesianProductFn::<HashSet<_>>::default());

        check_lattice_bimorphism(func, items_a, items_b);
    }
}
