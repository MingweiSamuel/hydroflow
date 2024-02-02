use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use hydroflow::{assert_graphvis_snapshots, hydroflow_syntax};
use hydroflow_macro::hydroflow_parser;
use lattices::map_union::{MapUnionHashMap, MapUnionSingletonMap};
use lattices::set_union::{SetUnionHashSet, SetUnionSingletonSet};
use multiplatform_test::multiplatform_test;

#[multiplatform_test]
pub fn tick_static() {
    type Item = MapUnionSingletonMap<u32, SetUnionSingletonSet<u32>>;
    type State = MapUnionHashMap<u32, SetUnionHashSet<u32>>;

    fn to_lattice_item((k, v): (u32, u32)) -> Item {
        MapUnionSingletonMap::new_from((k, SetUnionSingletonSet::new_from(v)))
    }

    let mut df = hydroflow_syntax! {
        x = source_iter([(7, 1), (7, 2)]) -> map(to_lattice_item) -> state::<State>();

        x[items] -> [items]dbg;
        x[state] -> [state]dbg;

        dbg = state_debug();
    };

    df.run_available();
}
