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
    let mut df = hydroflow_syntax! {
        lhs = source_iter_delta([(7, 1), (7, 2)])
            -> map(|(k, v)| MapUnionSingletonMap::new_from((k, SetUnionSingletonSet::new_from(v))))
            -> state::<MapUnionHashMap<usize, SetUnionHashSet<usize>>>();
        rhs = source_iter_delta([(7, 0), (7, 1), (7, 2)])
            -> map(|(k, v)| MapUnionSingletonMap::new_from((k, SetUnionSingletonSet::new_from(v))))
            -> state::<MapUnionHashMap<usize, SetUnionHashSet<usize>>>();

        lhs[items] -> [items_0]my_join;
        rhs[items] -> [items_1]my_join;
        lhs[state] -> [state_0]my_join;
        rhs[state] -> [state_1]my_join;

        my_join = state_join()
            -> for_each(|x| println!("OUT: {:?}", x));
    };

    // assert_graphvis_snapshots!(df);
    df.run_available();
}
