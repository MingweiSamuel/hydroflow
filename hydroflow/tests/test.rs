#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use hydroflow_macro::{hydroflow_syntax, DemuxEnum};
use multiplatform_test::multiplatform_test;
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_demux_enum"]
pub const test_demux_enum: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_demux_enum"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "hydroflow/tests/surface_demux_enum.rs",
        start_line: 5usize,
        start_col: 8usize,
        end_line: 5usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(test_demux_enum())),
};
#[no_mangle]
pub extern "C" fn __wbgt_test_demux_enum_0(cx: &::wasm_bindgen_test::__rt::Context) {
    let test_name = "surface_demux_enum::test_demux_enum";
    cx.execute_sync(test_name, test_demux_enum);
}
pub fn test_demux_enum() {
    enum Shapes {
        Square(f32),
        Rectangle { w: f32, h: f32 },
        Circle { r: f32 },
    }
    impl<__PusheratorCircle, __PusheratorRectangle, __PusheratorSquare>
        ::hydroflow::util::demux_enum::DemuxEnum<(
            __PusheratorCircle,
            (__PusheratorRectangle, (__PusheratorSquare, ())),
        )> for Shapes
    where
        __PusheratorCircle: ::hydroflow::pusherator::Pusherator<Item = (f32,)>,
        __PusheratorRectangle: ::hydroflow::pusherator::Pusherator<Item = (f32, f32)>,
        __PusheratorSquare: ::hydroflow::pusherator::Pusherator<Item = f32>,
    {
        fn demux(
            self,
            (mut __pusherator_circle , (mut __pusherator_rectangle , (mut __pusherator_square , ()))) : (__PusheratorCircle , (__PusheratorRectangle , (__PusheratorSquare , ()))),
        ) {
            match self {
                Self::Circle { r } => __pusherator_circle.give((r,)),
                Self::Rectangle { w, h } => __pusherator_rectangle.give((w, h)),
                Self::Square(_0) => __pusherator_square.give((_0)),
            }
        }
    }
    let mut df = {
        #[allow(unused_qualifications)]
        {
            use ::hydroflow::{var_expr, var_args};
            let mut df = ::hydroflow::scheduled::graph::Hydroflow::new();
            df . __assign_meta_graph ("{\"nodes\":[{\"value\":null,\"version\":0},{\"value\":{\"Operator\":\"source_iter([Shape :: Rectangle { width : 10.0, height : 8.0 }, Shape ::\\nSquare(9.0), Shape :: Circle { r : 5.0 },])\"},\"version\":1},{\"value\":{\"Operator\":\"demux_enum()\"},\"version\":1},{\"value\":{\"Operator\":\"map(| (r,) | std :: f64 :: consts :: PI * r * r)\"},\"version\":1},{\"value\":{\"Operator\":\"map(| (w, h) | w * h)\"},\"version\":1},{\"value\":{\"Operator\":\"map(| s | s * s)\"},\"version\":1},{\"value\":{\"Operator\":\"union()\"},\"version\":1},{\"value\":{\"Operator\":\"for_each(| area | println! (\\\"Area: {}\\\", area))\"},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1}],\"graph\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":8,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":4,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":9,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":10,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":6,\"version\":1},{\"idx\":7,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":8,\"version\":1},{\"idx\":3,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":9,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":10,\"version\":1},{\"idx\":5,\"version\":1}],\"version\":1}],\"ports\":[{\"value\":null,\"version\":0},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[{\"Path\":\"Circle\"},\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[{\"Path\":\"Rectangle\"},\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[{\"Path\":\"Square\"},\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1}],\"node_subgraph\":[{\"value\":null,\"version\":0},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1}],\"subgraph_nodes\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1},{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1},{\"idx\":7,\"version\":1}],\"version\":1}],\"subgraph_stratum\":[{\"value\":null,\"version\":0},{\"value\":0,\"version\":1},{\"value\":0,\"version\":1}],\"node_varnames\":[{\"value\":null,\"version\":0},{\"value\":\"my_demux\",\"version\":1},{\"value\":\"my_demux\",\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":\"out\",\"version\":1},{\"value\":\"out\",\"version\":1}],\"flow_props\":[{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":8,\"lattice_flow_type\":null},\"version\":1}]}") ;
            df.__assign_diagnostics("[]");
            let (hoff_8v1_send, hoff_8v1_recv) = df
                .make_edge::<_, ::hydroflow::scheduled::handoff::VecHandoff<_>>(
                    "handoff GraphNodeId(8v1)",
                );
            let (hoff_9v1_send, hoff_9v1_recv) = df
                .make_edge::<_, ::hydroflow::scheduled::handoff::VecHandoff<_>>(
                    "handoff GraphNodeId(9v1)",
                );
            let (hoff_10v1_send, hoff_10v1_recv) = df
                .make_edge::<_, ::hydroflow::scheduled::handoff::VecHandoff<_>>(
                    "handoff GraphNodeId(10v1)",
                );
            let mut sg_1v1_node_1v1_iter = {
                #[inline(always)]
                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item>, Item>(
                    into_iter: IntoIter,
                ) -> impl ::std::iter::Iterator<Item = Item> {
                    ::std::iter::IntoIterator::into_iter(into_iter)
                }
                check_iter([
                    Shape::Rectangle {
                        width: 10.0,
                        height: 8.0,
                    },
                    Shape::Square(9.0),
                    Shape::Circle { r: 5.0 },
                ])
            };
            df . add_subgraph_stratified ("Subgraph GraphSubgraphId(1v1)" , 0 , () , (hoff_8v1_send , (hoff_9v1_send , (hoff_10v1_send , ()))) , move | context , () , (hoff_8v1_send , (hoff_9v1_send , (hoff_10v1_send , ()))) | { let hoff_8v1_send = :: hydroflow :: pusherator :: for_each :: ForEach :: new (| v | { hoff_8v1_send . give (Some (v)) ; }) ; let hoff_9v1_send = :: hydroflow :: pusherator :: for_each :: ForEach :: new (| v | { hoff_9v1_send . give (Some (v)) ; }) ; let hoff_10v1_send = :: hydroflow :: pusherator :: for_each :: ForEach :: new (| v | { hoff_10v1_send . give (Some (v)) ; }) ; let op_1v1 = sg_1v1_node_1v1_iter . by_ref () ; let op_1v1 = { # [allow (non_snake_case)] # [inline (always)] pub fn op_1v1__source_iter__loc_unknown_start_0_0_end_0_0 < Item , Input : :: std :: iter :: Iterator < Item = Item > > (input : Input) -> impl :: std :: iter :: Iterator < Item = Item > { struct Pull < Item , Input : :: std :: iter :: Iterator < Item = Item > > { inner : Input , } impl < Item , Input : :: std :: iter :: Iterator < Item = Item > > Iterator for Pull < Item , Input > { type Item = Item ; # [inline (always)] fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } } Pull { inner : input } } op_1v1__source_iter__loc_unknown_start_0_0_end_0_0 (op_1v1) } ; let op_2v1 = { # [allow (unused_imports)] use :: hydroflow :: pusherator :: Pusherator ; :: hydroflow :: pusherator :: demux :: Demux :: new (< _ as :: hydroflow :: util :: demux_enum :: DemuxEnum > :: demux_enum , (hoff_8v1_send , (hoff_9v1_send , (hoff_10v1_send , ())))) } ; let op_2v1 = { # [allow (non_snake_case)] # [inline (always)] pub fn op_2v1__demux_enum__loc_unknown_start_0_0_end_0_0 < Item , Input : :: hydroflow :: pusherator :: Pusherator < Item = Item > > (input : Input) -> impl :: hydroflow :: pusherator :: Pusherator < Item = Item > { struct Push < Item , Input : :: hydroflow :: pusherator :: Pusherator < Item = Item > > { inner : Input , } impl < Item , Input : :: hydroflow :: pusherator :: Pusherator < Item = Item > > :: hydroflow :: pusherator :: Pusherator for Push < Item , Input > { type Item = Item ; # [inline (always)] fn give (& mut self , item : Self :: Item) { self . inner . give (item) } } Push { inner : input } } op_2v1__demux_enum__loc_unknown_start_0_0_end_0_0 (op_2v1) } ; # [inline (always)] fn check_pivot_run < Pull : :: std :: iter :: Iterator < Item = Item > , Push : :: hydroflow :: pusherator :: Pusherator < Item = Item > , Item > (pull : Pull , push : Push) { :: hydroflow :: pusherator :: pivot :: Pivot :: new (pull , push) . run () ; } check_pivot_run (op_1v1 , op_2v1) ; }) ;
            df . add_subgraph_stratified ("Subgraph GraphSubgraphId(2v1)" , 0 , (hoff_8v1_recv , (hoff_9v1_recv , (hoff_10v1_recv , ()))) , () , move | context , (hoff_8v1_recv , (hoff_9v1_recv , (hoff_10v1_recv , ()))) , () | { let mut hoff_8v1_recv = hoff_8v1_recv . borrow_mut_swap () ; let hoff_8v1_recv = hoff_8v1_recv . drain (..) ; let mut hoff_9v1_recv = hoff_9v1_recv . borrow_mut_swap () ; let hoff_9v1_recv = hoff_9v1_recv . drain (..) ; let mut hoff_10v1_recv = hoff_10v1_recv . borrow_mut_swap () ; let hoff_10v1_recv = hoff_10v1_recv . drain (..) ; let op_3v1 = hoff_8v1_recv . map (| (r ,) | std :: f64 :: consts :: PI * r * r) ; let op_3v1 = { # [allow (non_snake_case)] # [inline (always)] pub fn op_3v1__map__loc_unknown_start_0_0_end_0_0 < Item , Input : :: std :: iter :: Iterator < Item = Item > > (input : Input) -> impl :: std :: iter :: Iterator < Item = Item > { struct Pull < Item , Input : :: std :: iter :: Iterator < Item = Item > > { inner : Input , } impl < Item , Input : :: std :: iter :: Iterator < Item = Item > > Iterator for Pull < Item , Input > { type Item = Item ; # [inline (always)] fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } } Pull { inner : input } } op_3v1__map__loc_unknown_start_0_0_end_0_0 (op_3v1) } ; let op_4v1 = hoff_9v1_recv . map (| (w , h) | w * h) ; let op_4v1 = { # [allow (non_snake_case)] # [inline (always)] pub fn op_4v1__map__loc_unknown_start_0_0_end_0_0 < Item , Input : :: std :: iter :: Iterator < Item = Item > > (input : Input) -> impl :: std :: iter :: Iterator < Item = Item > { struct Pull < Item , Input : :: std :: iter :: Iterator < Item = Item > > { inner : Input , } impl < Item , Input : :: std :: iter :: Iterator < Item = Item > > Iterator for Pull < Item , Input > { type Item = Item ; # [inline (always)] fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } } Pull { inner : input } } op_4v1__map__loc_unknown_start_0_0_end_0_0 (op_4v1) } ; let op_5v1 = hoff_10v1_recv . map (| s | s * s) ; let op_5v1 = { # [allow (non_snake_case)] # [inline (always)] pub fn op_5v1__map__loc_unknown_start_0_0_end_0_0 < Item , Input : :: std :: iter :: Iterator < Item = Item > > (input : Input) -> impl :: std :: iter :: Iterator < Item = Item > { struct Pull < Item , Input : :: std :: iter :: Iterator < Item = Item > > { inner : Input , } impl < Item , Input : :: std :: iter :: Iterator < Item = Item > > Iterator for Pull < Item , Input > { type Item = Item ; # [inline (always)] fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } } Pull { inner : input } } op_5v1__map__loc_unknown_start_0_0_end_0_0 (op_5v1) } ; let op_6v1 = { # [allow (unused)] # [inline (always)] fn check_inputs < A : :: std :: iter :: Iterator < Item = Item > , B : :: std :: iter :: Iterator < Item = Item > , Item > (a : A , b : B) -> impl :: std :: iter :: Iterator < Item = Item > { a . chain (b) } check_inputs (check_inputs (op_3v1 , op_4v1) , op_5v1) } ; let op_6v1 = { # [allow (non_snake_case)] # [inline (always)] pub fn op_6v1__union__loc_unknown_start_0_0_end_0_0 < Item , Input : :: std :: iter :: Iterator < Item = Item > > (input : Input) -> impl :: std :: iter :: Iterator < Item = Item > { struct Pull < Item , Input : :: std :: iter :: Iterator < Item = Item > > { inner : Input , } impl < Item , Input : :: std :: iter :: Iterator < Item = Item > > Iterator for Pull < Item , Input > { type Item = Item ; # [inline (always)] fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } } Pull { inner : input } } op_6v1__union__loc_unknown_start_0_0_end_0_0 (op_6v1) } ; let op_7v1 = :: hydroflow :: pusherator :: for_each :: ForEach :: new (| area | { :: std :: io :: _print (format_args ! ("Area: {0}\n" , area)) ; }) ; let op_7v1 = { # [allow (non_snake_case)] # [inline (always)] pub fn op_7v1__for_each__loc_unknown_start_0_0_end_0_0 < Item , Input : :: hydroflow :: pusherator :: Pusherator < Item = Item > > (input : Input) -> impl :: hydroflow :: pusherator :: Pusherator < Item = Item > { struct Push < Item , Input : :: hydroflow :: pusherator :: Pusherator < Item = Item > > { inner : Input , } impl < Item , Input : :: hydroflow :: pusherator :: Pusherator < Item = Item > > :: hydroflow :: pusherator :: Pusherator for Push < Item , Input > { type Item = Item ; # [inline (always)] fn give (& mut self , item : Self :: Item) { self . inner . give (item) } } Push { inner : input } } op_7v1__for_each__loc_unknown_start_0_0_end_0_0 (op_7v1) } ; # [inline (always)] fn check_pivot_run < Pull : :: std :: iter :: Iterator < Item = Item > , Push : :: hydroflow :: pusherator :: Pusherator < Item = Item > , Item > (pull : Pull , push : Push) { :: hydroflow :: pusherator :: pivot :: Pivot :: new (pull , push) . run () ; } check_pivot_run (op_6v1 , op_7v1) ; }) ;
            df
        }
    };
    df.run_available();
}
#[rustc_main]
#[no_coverage]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_demux_enum])
}
