use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use hydroflow::{assert_graphvis_snapshots, hydroflow_syntax};
use hydroflow_macro::hydroflow_parser;
use lattices::map_union::{MapUnionHashMap, MapUnionSingletonMap};
use lattices::set_union::{SetUnionHashSet, SetUnionSingletonSet};
use multiplatform_test::multiplatform_test;

#[test]
pub fn tick_static() {
    let mut df = {
        #[allow(unused_qualifications)]
        {
            use ::hydroflow::{var_expr, var_args};
            let mut df = ::hydroflow::scheduled::graph::Hydroflow::new();
            df.__assign_meta_graph(
                "{\"nodes\":[{\"value\":null,\"version\":0},{\"value\":{\"Operator\":\"source_iter_delta([(7, 1), (7, 2)])\"},\"version\":1},{\"value\":{\"Operator\":\"map(| (k, v) | MapUnionSingletonMap ::\\nnew_from((k, SetUnionSingletonSet :: new_from(v))))\"},\"version\":1},{\"value\":{\"Operator\":\"state :: < MapUnionHashMap < usize, SetUnionHashSet < usize > > > ()\"},\"version\":1},{\"value\":{\"Operator\":\"source_iter_delta([(7, 0), (7, 1), (7, 2)])\"},\"version\":1},{\"value\":{\"Operator\":\"map(| (k, v) | MapUnionSingletonMap ::\\nnew_from((k, SetUnionSingletonSet :: new_from(v))))\"},\"version\":1},{\"value\":{\"Operator\":\"state :: < MapUnionHashMap < usize, SetUnionHashSet < usize > > > ()\"},\"version\":1},{\"value\":{\"Operator\":\"state_join()\"},\"version\":1},{\"value\":{\"Operator\":\"for_each(| x | println! (\\\"OUT: {:?}\\\", x))\"},\"version\":1}],\"edge_types\":[{\"value\":null,\"version\":0},{\"value\":\"Value\",\"version\":1},{\"value\":\"Value\",\"version\":1},{\"value\":\"Value\",\"version\":1},{\"value\":\"Value\",\"version\":1},{\"value\":\"Value\",\"version\":1},{\"value\":\"Value\",\"version\":1},{\"value\":\"Reference\",\"version\":1},{\"value\":\"Reference\",\"version\":1},{\"value\":\"Value\",\"version\":1}],\"graph\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":3,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":4,\"version\":1},{\"idx\":5,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":7,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":6,\"version\":1},{\"idx\":7,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":7,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":6,\"version\":1},{\"idx\":7,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":7,\"version\":1},{\"idx\":8,\"version\":1}],\"version\":1}],\"ports\":[{\"value\":null,\"version\":0},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[{\"Path\":\"items\"},{\"Path\":\"items_0\"}],\"version\":1},{\"value\":[{\"Path\":\"items\"},{\"Path\":\"items_1\"}],\"version\":1},{\"value\":[{\"Path\":\"state\"},{\"Path\":\"state_0\"}],\"version\":1},{\"value\":[{\"Path\":\"state\"},{\"Path\":\"state_1\"}],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1}],\"node_subgraph\":[{\"value\":null,\"version\":0},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1}],\"subgraph_nodes\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1},{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1},{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1},{\"idx\":7,\"version\":1},{\"idx\":8,\"version\":1}],\"version\":1}],\"subgraph_stratum\":[{\"value\":null,\"version\":0},{\"value\":0,\"version\":1}],\"node_varnames\":[{\"value\":null,\"version\":0},{\"value\":\"lhs\",\"version\":1},{\"value\":\"lhs\",\"version\":1},{\"value\":\"lhs\",\"version\":1},{\"value\":\"rhs\",\"version\":1},{\"value\":\"rhs\",\"version\":1},{\"value\":\"rhs\",\"version\":1},{\"value\":\"my_join\",\"version\":1},{\"value\":\"my_join\",\"version\":1}],\"flow_props\":[{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":0,\"lattice_flow_type\":\"Delta\"},\"version\":1},{\"value\":{\"star_ord\":0,\"lattice_flow_type\":\"Delta\"},\"version\":1},{\"value\":{\"star_ord\":2,\"lattice_flow_type\":\"Delta\"},\"version\":1},{\"value\":{\"star_ord\":2,\"lattice_flow_type\":\"Delta\"},\"version\":1},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":4,\"lattice_flow_type\":null},\"version\":1}],\"subgraph_laziness\":[{\"value\":null,\"version\":0}]}",
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
            let edge_7v1 = df
                .add_state(
                    ::std::cell::RefCell::new(
                        <MapUnionHashMap<
                            usize,
                            SetUnionHashSet<usize>,
                        > as ::std::default::Default>::default(),
                    ),
                );
            let mut sg_1v1_node_4v1_iter = {
                #[inline(always)]
                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item>, Item>(
                    into_iter: IntoIter,
                ) -> impl ::std::iter::Iterator<Item = Item> {
                    ::std::iter::IntoIterator::into_iter(into_iter)
                }
                check_iter([(7, 0), (7, 1), (7, 2)])
            };
            let edge_8v1 = df
                .add_state(
                    ::std::cell::RefCell::new(
                        <MapUnionHashMap<
                            usize,
                            SetUnionHashSet<usize>,
                        > as ::std::default::Default>::default(),
                    ),
                );
            df.add_subgraph_stratified(
                "Subgraph GraphSubgraphId(1v1)",
                0,
                (),
                (),
                false,
                move |context, (), ()| {
                    let op_1v1 = sg_1v1_node_1v1_iter.by_ref();
                    let op_1v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_1v1__source_iter_delta__loc_unknown_start_0_0_end_0_0<
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
                        op_1v1__source_iter_delta__loc_unknown_start_0_0_end_0_0(op_1v1)
                    };
                    let op_2v1 = op_1v1
                        .map(|(k, v)| MapUnionSingletonMap::new_from((
                            k,
                            SetUnionSingletonSet::new_from(v),
                        )));
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
                            state_handle: ::hydroflow::scheduled::state::StateHandle<
                                ::std::cell::RefCell<Lat>,
                            >,
                            context: &'a ::hydroflow::scheduled::context::Context,
                        ) -> impl 'a + ::std::iter::Iterator<Item = Item>
                        where
                            Item: ::std::clone::Clone,
                            Iter: 'a + ::std::iter::Iterator<Item = Item>,
                            Lat: 'static + ::hydroflow::lattices::Merge<Item>,
                        {
                            iter.inspect(move |item| {
                                let state = context.state_ref(state_handle);
                                let mut state = state.borrow_mut();
                                ::hydroflow::lattices::Merge::merge(
                                    &mut *state,
                                    ::std::clone::Clone::clone(item),
                                );
                            })
                        }
                        check_input::<
                            _,
                            _,
                            MapUnionHashMap<usize, SetUnionHashSet<usize>>,
                        >(op_2v1, edge_7v1, context)
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
                    let op_4v1 = sg_1v1_node_4v1_iter.by_ref();
                    let op_4v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_4v1__source_iter_delta__loc_unknown_start_0_0_end_0_0<
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
                        op_4v1__source_iter_delta__loc_unknown_start_0_0_end_0_0(op_4v1)
                    };
                    let op_5v1 = op_4v1
                        .map(|(k, v)| MapUnionSingletonMap::new_from((
                            k,
                            SetUnionSingletonSet::new_from(v),
                        )));
                    let op_5v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_5v1__map__loc_unknown_start_0_0_end_0_0<
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
                        op_5v1__map__loc_unknown_start_0_0_end_0_0(op_5v1)
                    };
                    let op_6v1 = {
                        fn check_input<'a, Item, Iter, Lat>(
                            iter: Iter,
                            state_handle: ::hydroflow::scheduled::state::StateHandle<
                                ::std::cell::RefCell<Lat>,
                            >,
                            context: &'a ::hydroflow::scheduled::context::Context,
                        ) -> impl 'a + ::std::iter::Iterator<Item = Item>
                        where
                            Item: ::std::clone::Clone,
                            Iter: 'a + ::std::iter::Iterator<Item = Item>,
                            Lat: 'static + ::hydroflow::lattices::Merge<Item>,
                        {
                            iter.inspect(move |item| {
                                let state = context.state_ref(state_handle);
                                let mut state = state.borrow_mut();
                                ::hydroflow::lattices::Merge::merge(
                                    &mut *state,
                                    ::std::clone::Clone::clone(item),
                                );
                            })
                        }
                        check_input::<
                            _,
                            _,
                            MapUnionHashMap<usize, SetUnionHashSet<usize>>,
                        >(op_5v1, edge_8v1, context)
                    };
                    let op_6v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_6v1__state__loc_unknown_start_0_0_end_0_0<
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
                        op_6v1__state__loc_unknown_start_0_0_end_0_0(op_6v1)
                    };
                    let op_7v1 = {
                        type __StateHandle<Map> = ::hydroflow::scheduled::state::StateHandle<
                            ::std::cell::RefCell<
                                ::hydroflow::lattices::map_union::MapUnion<Map>,
                            >,
                        >;
                        #[inline(always)]
                        fn check_inputs<'a, LhsState, RhsState, Key, LhsVal, RhsVal>(
                            context: &'a ::hydroflow::scheduled::context::Context,
                            lhs_state_handle: __StateHandle<LhsState>,
                            rhs_state_handle: __StateHandle<RhsState>,
                        ) -> impl 'a + Iterator<Item = ()>
                        where
                            LhsState: ::hydroflow::lattices::cc_traits::MapMut<
                                Key,
                                LhsVal,
                                Key = Key,
                                Item = LhsVal,
                            >,
                            RhsState: ::hydroflow::lattices::cc_traits::MapMut<
                                Key,
                                RhsVal,
                                Key = Key,
                                Item = RhsVal,
                            >,
                        {
                            panic!("not yet implemented");
                            ::core::iter::empty()
                        }
                        check_inputs(&context, edge_7v1, edge_8v1)
                    };
                    let op_7v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_7v1__state_join__loc_unknown_start_0_0_end_0_0<
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
                        op_7v1__state_join__loc_unknown_start_0_0_end_0_0(op_7v1)
                    };
                    let op_8v1 = ::hydroflow::pusherator::for_each::ForEach::new(|x| {
                        println!("OUT: {0:?}\n", x);
                    });
                    let op_8v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_8v1__for_each__loc_unknown_start_0_0_end_0_0<
                            Item,
                            Input: ::hydroflow::pusherator::Pusherator<Item = Item>,
                        >(
                            input: Input,
                        ) -> impl ::hydroflow::pusherator::Pusherator<Item = Item> {
                            struct Push<
                                Item,
                                Input: ::hydroflow::pusherator::Pusherator<Item = Item>,
                            > {
                                inner: Input,
                            }
                            impl<
                                Item,
                                Input: ::hydroflow::pusherator::Pusherator<Item = Item>,
                            > ::hydroflow::pusherator::Pusherator for Push<Item, Input> {
                                type Item = Item;
                                #[inline(always)]
                                fn give(&mut self, item: Self::Item) {
                                    self.inner.give(item)
                                }
                            }
                            Push { inner: input }
                        }
                        op_8v1__for_each__loc_unknown_start_0_0_end_0_0(op_8v1)
                    };
                    #[inline(always)]
                    fn check_pivot_run<
                        Pull: ::std::iter::Iterator<Item = Item>,
                        Push: ::hydroflow::pusherator::Pusherator<Item = Item>,
                        Item,
                    >(pull: Pull, push: Push) {
                        ::hydroflow::pusherator::pivot::Pivot::new(pull, push).run();
                    }
                    check_pivot_run(op_7v1, op_8v1);
                },
            );
            df
        }
    };
    df.run_available();
}

