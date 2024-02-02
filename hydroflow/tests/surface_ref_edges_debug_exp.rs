use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use hydroflow::{assert_graphvis_snapshots, hydroflow_syntax};
use hydroflow_macro::hydroflow_parser;
use lattices::map_union::{MapUnionHashMap, MapUnionSingletonMap};
use lattices::set_union::{SetUnionHashSet, SetUnionSingletonSet};
use multiplatform_test::multiplatform_test;

type Item = MapUnionSingletonMap<u32, SetUnionSingletonSet<u32>>;
type State = MapUnionHashMap<u32, SetUnionHashSet<u32>>;

fn to_lattice_item((k, v): (u32, u32)) -> Item {
    MapUnionSingletonMap::new_from((k, SetUnionSingletonSet::new_from(v)))
}
fn main() {
    let mut df = {
        #[allow(unused_qualifications)]
        {
            use hydroflow::{var_expr, var_args};
            let mut df = hydroflow::scheduled::graph::Hydroflow::new();
            df.__assign_meta_graph(
                "{\"nodes\":[{\"value\":null,\"version\":0},{\"value\":{\"Operator\":\"source_iter([(7, 1), (7, 2)])\"},\"version\":1},{\"value\":{\"Operator\":\"map(to_lattice_item)\"},\"version\":1},{\"value\":{\"Operator\":\"state :: < State > ()\"},\"version\":1},{\"value\":{\"Operator\":\"state_debug()\"},\"version\":1}],\"edge_types\":[{\"value\":null,\"version\":0},{\"value\":\"Value\",\"version\":1},{\"value\":\"Value\",\"version\":1},{\"value\":\"Value\",\"version\":1},{\"value\":\"Reference\",\"version\":1}],\"graph\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":3,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1}],\"ports\":[{\"value\":null,\"version\":0},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[{\"Path\":\"items\"},{\"Path\":\"items\"}],\"version\":1},{\"value\":[{\"Path\":\"state\"},{\"Path\":\"state\"}],\"version\":1}],\"node_subgraph\":[{\"value\":null,\"version\":0},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1}],\"subgraph_nodes\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1},{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1}],\"subgraph_stratum\":[{\"value\":null,\"version\":0},{\"value\":0,\"version\":1}],\"node_varnames\":[{\"value\":null,\"version\":0},{\"value\":\"x\",\"version\":1},{\"value\":\"x\",\"version\":1},{\"value\":\"x\",\"version\":1},{\"value\":\"dbg\",\"version\":1}],\"flow_props\":[{\"value\":null,\"version\":0}],\"subgraph_laziness\":[{\"value\":null,\"version\":0}]}",
            );
            df.__assign_diagnostics("[]");
            let mut sg_1v1_node_1v1_iter = {
                #[inline(always)]
                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item>, Item>(
                    into_iter: IntoIter,
                ) -> impl ::std::iter::Iterator<Item = Item> {
                    ::std::iter::IntoIterator::into_iter(into_iter)
                }
                check_iter([(7, 1), (7, 2)])
            };
            let edge_4v1 = df
                .add_state(
                    ::std::cell::RefCell::new(
                        <State as ::std::default::Default>::default(),
                    ),
                );
            df.add_subgraph_stratified(
                "Subgraph GraphSubgraphId(1v1)",
                0,
                var_expr!(),
                var_expr!(),
                false,
                move |context, var_args!(), var_args!()| {
                    let op_1v1 = sg_1v1_node_1v1_iter.by_ref();
                    let op_1v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_1v1__source_iter__loc_unknown_start_0_0_end_0_0<
                            Item,
                            Input: ::std::iter::Iterator<Item = Item>,
                        >(input: Input) -> impl ::std::iter::Iterator<Item = Item> {
                            struct Pull<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > {
                                inner: Input,
                            }
                            impl<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > Iterator for Pull<Item, Input> {
                                type Item = Item;
                                #[inline(always)]
                                fn next(&mut self) -> Option<Self::Item> {
                                    self.inner.next()
                                }
                                #[inline(always)]
                                fn size_hint(&self) -> (usize, Option<usize>) {
                                    self.inner.size_hint()
                                }
                            }
                            Pull { inner: input }
                        }
                        op_1v1__source_iter__loc_unknown_start_0_0_end_0_0(op_1v1)
                    };
                    let op_2v1 = op_1v1.map(to_lattice_item);
                    let op_2v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_2v1__map__loc_unknown_start_0_0_end_0_0<
                            Item,
                            Input: ::std::iter::Iterator<Item = Item>,
                        >(input: Input) -> impl ::std::iter::Iterator<Item = Item> {
                            struct Pull<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > {
                                inner: Input,
                            }
                            impl<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > Iterator for Pull<Item, Input> {
                                type Item = Item;
                                #[inline(always)]
                                fn next(&mut self) -> Option<Self::Item> {
                                    self.inner.next()
                                }
                                #[inline(always)]
                                fn size_hint(&self) -> (usize, Option<usize>) {
                                    self.inner.size_hint()
                                }
                            }
                            Pull { inner: input }
                        }
                        op_2v1__map__loc_unknown_start_0_0_end_0_0(op_2v1)
                    };
                    let op_3v1 = {
                        fn check_input<'a, Item, Iter, Lat>(
                            iter: Iter,
                            state_handle: hydroflow::scheduled::state::StateHandle<
                                ::std::cell::RefCell<Lat>,
                            >,
                            context: &'a hydroflow::scheduled::context::Context,
                        ) -> impl 'a + ::std::iter::Iterator<Item = Item>
                        where
                            Item: ::std::clone::Clone,
                            Iter: 'a + ::std::iter::Iterator<Item = Item>,
                            Lat: 'static + hydroflow::lattices::Merge<Item>,
                        {
                            iter.inspect(move |item| {
                                let state = context.state_ref(state_handle);
                                let mut state = state.borrow_mut();
                                hydroflow::lattices::Merge::merge(
                                    &mut *state,
                                    ::std::clone::Clone::clone(item),
                                );
                            })
                        }
                        check_input::<_, _, State>(op_2v1, edge_4v1, context)
                    };
                    let op_3v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_3v1__state__loc_unknown_start_0_0_end_0_0<
                            Item,
                            Input: ::std::iter::Iterator<Item = Item>,
                        >(input: Input) -> impl ::std::iter::Iterator<Item = Item> {
                            struct Pull<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > {
                                inner: Input,
                            }
                            impl<
                                Item,
                                Input: ::std::iter::Iterator<Item = Item>,
                            > Iterator for Pull<Item, Input> {
                                type Item = Item;
                                #[inline(always)]
                                fn next(&mut self) -> Option<Self::Item> {
                                    self.inner.next()
                                }
                                #[inline(always)]
                                fn size_hint(&self) -> (usize, Option<usize>) {
                                    self.inner.size_hint()
                                }
                            }
                            Pull { inner: input }
                        }
                        op_3v1__state__loc_unknown_start_0_0_end_0_0(op_3v1)
                    };
                    let op_4v1 = hydroflow::pusherator::for_each::ForEach::new(|item| {
                        println!(
                            "ITEM {:?}; STATE {:?}",
                            item,
                            &*context.state_ref(edge_4v1).borrow(),
                        );
                    });
                    let op_4v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_4v1__state_debug__loc_unknown_start_0_0_end_0_0<
                            Item,
                            Input: hydroflow::pusherator::Pusherator<Item = Item>,
                        >(
                            input: Input,
                        ) -> impl hydroflow::pusherator::Pusherator<Item = Item> {
                            struct Push<
                                Item,
                                Input: hydroflow::pusherator::Pusherator<Item = Item>,
                            > {
                                inner: Input,
                            }
                            impl<
                                Item,
                                Input: hydroflow::pusherator::Pusherator<Item = Item>,
                            > hydroflow::pusherator::Pusherator for Push<Item, Input> {
                                type Item = Item;
                                #[inline(always)]
                                fn give(&mut self, item: Self::Item) {
                                    self.inner.give(item)
                                }
                            }
                            Push { inner: input }
                        }
                        op_4v1__state_debug__loc_unknown_start_0_0_end_0_0(op_4v1)
                    };
                    #[inline(always)]
                    fn check_pivot_run<
                        Pull: ::std::iter::Iterator<Item = Item>,
                        Push: hydroflow::pusherator::Pusherator<Item = Item>,
                        Item,
                    >(pull: Pull, push: Push) {
                        hydroflow::pusherator::pivot::Pivot::new(pull, push).run();
                    }
                    check_pivot_run(op_3v1, op_4v1);
                },
            );
            df
        }
    };
    df.run_available();
}
