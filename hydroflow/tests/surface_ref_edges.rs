use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use hydroflow::{assert_graphvis_snapshots, hydroflow_syntax};
use hydroflow_macro::hydroflow_parser;
use multiplatform_test::multiplatform_test;

#[multiplatform_test]
pub fn tick_static() {
    // let mut df =
    hydroflow_parser! {
        lhs = source_iter([(7, 1), (7, 2)]) -> state();
        rhs = source_iter([(7, 0), (7, 1), (7, 2)]) -> state();

        lhs[items] -> [items_0]my_join;
        rhs[items] -> [items_1]my_join;
        lhs[state] -> [state_0]my_join;
        rhs[state] -> [state_1]my_join;

        my_join = state_join()
            -> for_each(|x| println!("OUT: {:?}", x));
    }
    //;
    // assert_graphvis_snapshots!(df);
    // df.run_available();
}
