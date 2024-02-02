#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use hydroflow::{assert_graphvis_snapshots, hydroflow_syntax};
use hydroflow_macro::hydroflow_parser;
use multiplatform_test::multiplatform_test;
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "tick_static"]
pub const tick_static: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("tick_static"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "hydroflow\\tests\\surface_ref_edges.rs",
        start_line: 10usize,
        start_col: 8usize,
        end_line: 10usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(tick_static())),
};
#[no_mangle]
pub extern "C" fn __wbgt_tick_static_0(cx: &::wasm_bindgen_test::__rt::Context) {
    let test_name = "surface_ref_edges::tick_static";
    cx.execute_sync(
        test_name,
        tick_static,
        ::core::option::Option::None,
        ::core::option::Option::None,
    );
}
pub fn tick_static() {
    {
        {
            ::std::io::_print(
                format_args!(
                    "{0}\n\n{1}\n\n",
                    "flowchart TB\n    %% hydroflow\\tests\\surface_ref_edges.rs:13:15\n    1v1[\"0:0 <tt>source_iter([(7, 1), (7, 2)])</tt>\"]\n    %% hydroflow\\tests\\surface_ref_edges.rs:13:48\n    2v1[\"0:0 <tt>state()</tt>\"]\n    %% hydroflow\\tests\\surface_ref_edges.rs:14:15\n    3v1[\"0:0 <tt>source_iter([(7, 0), (7, 1), (7, 2)])</tt>\"]\n    %% hydroflow\\tests\\surface_ref_edges.rs:14:56\n    4v1[\"0:0 <tt>state()</tt>\"]\n    %% hydroflow\\tests\\surface_ref_edges.rs:21:19\n    5v1[\"0:0 <tt>state_join()</tt>\"]\n    %% hydroflow\\tests\\surface_ref_edges.rs:22:16\n    6v1[\"0:0 <tt>for_each(| x | println! (&quot;OUT: {:?}&quot;, x))</tt>\"]\n\n    1v1-->2v1\n    3v1-->4v1\n    2v1-->5v1\n    4v1-->5v1\n    2v1-->5v1\n    4v1-->5v1\n    5v1-->6v1\n",
                    "%%{init:{'theme':'base','themeVariables':{'clusterBkg':'#ddd','clusterBorder':'#888'}}}%%\nflowchart TD\nclassDef pullClass fill:#8af,stroke:#000,text-align:left,white-space:pre\nclassDef pushClass fill:#ff8,stroke:#000,text-align:left,white-space:pre\nclassDef otherClass fill:#fdc,stroke:#000,text-align:left,white-space:pre\nlinkStyle default stroke:#aaa\n1v1[\\\"(1v1) <code>source_iter([(7, 1), (7, 2)])</code>\"/]:::pullClass\n2v1[\\\"(2v1) <code>state()</code>\"/]:::pullClass\n3v1[\\\"(3v1) <code>source_iter([(7, 0), (7, 1), (7, 2)])</code>\"/]:::pullClass\n4v1[\\\"(4v1) <code>state()</code>\"/]:::pullClass\n5v1[\\\"(5v1) <code>state_join()</code>\"/]:::pullClass\n6v1[/\"(6v1) <code>for_each(|x| println!(&quot;OUT: {:?}&quot;, x))</code>\"\\]:::pushClass\n1v1-->2v1\n3v1-->4v1\n2v1-->|items<br>items_0|5v1\n4v1-->|items<br>items_1|5v1\n2v1-->|state<br>state_0|5v1\n4v1-->|state<br>state_1|5v1\n5v1-->6v1\nsubgraph sg_1v1 [\"sg_1v1 stratum 0\"]\n    1v1\n    2v1\n    3v1\n    4v1\n    5v1\n    6v1\n    subgraph sg_1v1_var_lhs [\"var <tt>lhs</tt>\"]\n        1v1\n        2v1\n    end\n    subgraph sg_1v1_var_my_join [\"var <tt>my_join</tt>\"]\n        5v1\n        6v1\n    end\n    subgraph sg_1v1_var_rhs [\"var <tt>rhs</tt>\"]\n        3v1\n        4v1\n    end\nend\n"
                ),
            );
        };
    }
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&tick_static])
}
