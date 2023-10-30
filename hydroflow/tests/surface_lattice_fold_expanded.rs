use hydroflow::hydroflow_syntax;
use hydroflow::lattices::Max;

#[test]
fn test_basic() {
    let mut df = {
        #[allow(unused_qualifications)]
        {
            use ::hydroflow::{var_args, var_expr};
            let mut df = ::hydroflow::scheduled::graph::Hydroflow::new();
            df.__assign_meta_graph(
                "{\"nodes\":[{\"value\":null,\"version\":0},{\"value\":{\"Operator\":\"source_iter([1, 2, 3, 4, 5])\"},\"version\":1},{\"value\":{\"Operator\":\"map(Max :: new)\"},\"version\":1},{\"value\":{\"Operator\":\"lattice_fold :: < 'static > (| | Max :: < u32 > :: new(0))\"},\"version\":1},{\"value\":{\"Operator\":\"for_each(| x : Max < u32 > | println! (\\\"Least upper bound: {:?}\\\", x))\"},\"version\":1},{\"value\":{\"Handoff\":{}},\"version\":1}],\"graph\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":5,\"version\":1}],\"version\":3},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":5,\"version\":1},{\"idx\":3,\"version\":1}],\"version\":1}],\"ports\":[{\"value\":null,\"version\":0},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":3},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[\"Elided\",\"Elided\"],\"version\":1}],\"node_subgraph\":[{\"value\":null,\"version\":0},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1},{\"value\":{\"idx\":2,\"version\":1},\"version\":1}],\"subgraph_nodes\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1}],\"subgraph_stratum\":[{\"value\":null,\"version\":0},{\"value\":0,\"version\":1},{\"value\":1,\"version\":1}],\"node_varnames\":[{\"value\":null,\"version\":0}],\"flow_props\":[{\"value\":null,\"version\":0},{\"value\":{\"star_ord\":2,\"lattice_flow_type\":\"Cumul\"},\"version\":1}],\"subgraph_laziness\":[{\"value\":null,\"version\":0}]}",
            );
            df.__assign_diagnostics(
                "[{\"span\":{\"path\":\"hydroflow\\\\tests\\\\surface_lattice_fold.rs\",\"line\":0,\"column\":0},\"level\":\"Warning\",\"message\":\"`lattice_fold` expects lattice flow input, has sequential input. This may be an error in the future.\"},{\"span\":{\"path\":\"hydroflow\\\\tests\\\\surface_lattice_fold.rs\",\"line\":0,\"column\":0},\"level\":\"Warning\",\"message\":\"`lattice_fold` expects lattice flow input, has sequential input. This may be an error in the future.\"}]",
            );
            let (hoff_5v1_send, hoff_5v1_recv) = df
                .make_edge::<_, ::hydroflow::scheduled::handoff::VecHandoff<_>>(
                    "handoff GraphNodeId(5v1)",
                );
            let mut sg_1v1_node_1v1_iter = {
                #[inline(always)]
                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item>, Item>(
                    into_iter: IntoIter,
                ) -> impl ::std::iter::Iterator<Item = Item> {
                    ::std::iter::IntoIterator::into_iter(into_iter)
                }
                check_iter([1, 2, 3, 4, 5])
            };
            df.add_subgraph_stratified(
                "Subgraph GraphSubgraphId(1v1)",
                0,
                (),
                (hoff_5v1_send, ()),
                false,
                move |context, (), (hoff_5v1_send, ())| {
                    let hoff_5v1_send = ::hydroflow::pusherator::for_each::ForEach::new(|
                        v|
                    {
                        hoff_5v1_send.give(Some(v));
                    });
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
                    let op_2v1 = op_1v1.map(Max::new);
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
                    #[inline(always)]
                    fn check_pivot_run<
                        Pull: ::std::iter::Iterator<Item = Item>,
                        Push: ::hydroflow::pusherator::Pusherator<Item = Item>,
                        Item,
                    >(pull: Pull, push: Push) {
                        ::hydroflow::pusherator::pivot::Pivot::new(pull, push).run();
                    }
                    check_pivot_run(op_2v1, hoff_5v1_send);
                },
            );
            let sg_2v1_node_3v1_initializer_func = || Max::<u32>::new(0);
            #[allow(clippy::redundant_closure_call)]
            let sg_2v1_node_3v1_folddata = df.add_state(::std::cell::Cell::new(
                ::std::option::Option::Some((sg_2v1_node_3v1_initializer_func)()),
            ));
            df.add_subgraph_stratified(
                "Subgraph GraphSubgraphId(2v1)",
                1,
                (hoff_5v1_recv, ()),
                (),
                false,
                move |context, (hoff_5v1_recv, ()), ()| {
                    let mut hoff_5v1_recv = hoff_5v1_recv.borrow_mut_swap();
                    let hoff_5v1_recv = hoff_5v1_recv.drain(..);
                    let op_3v1 = {
                        let mut sg_2v1_node_3v1_accumulator = context
                            .state_ref(sg_2v1_node_3v1_folddata)
                            .take()
                            .expect("FOLD DATA MISSING");
                        for sg_2v1_node_3v1_iterator_item in hoff_5v1_recv {
                            #[allow(clippy::redundant_closure_call)]
                            (::hydroflow::lattices::Merge::merge)(
                                &mut sg_2v1_node_3v1_accumulator,
                                sg_2v1_node_3v1_iterator_item,
                            );
                        }
                        context
                            .state_ref(sg_2v1_node_3v1_folddata)
                            .set(
                                ::std::option::Option::Some(
                                    ::std::clone::Clone::clone(&sg_2v1_node_3v1_accumulator),
                                ),
                            );
                        ::std::iter::once(&sg_2v1_node_3v1_accumulator)
                    };
                    let op_3v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_3v1__lattice_fold__loc_unknown_start_0_0_end_0_0<
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
                        op_3v1__lattice_fold__loc_unknown_start_0_0_end_0_0(op_3v1)
                    };
                    let op_4v1 = ::hydroflow::pusherator::for_each::ForEach::new(|
                        x: &Max<u32>|
                    {
                        print!("Least upper bound: {0:?}\n", x);
                    });
                    let op_4v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_4v1__for_each__loc_unknown_start_0_0_end_0_0<
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
                        op_4v1__for_each__loc_unknown_start_0_0_end_0_0(op_4v1)
                    };
                    #[inline(always)]
                    fn check_pivot_run<
                        Pull: ::std::iter::Iterator<Item = Item>,
                        Push: ::hydroflow::pusherator::Pusherator<Item = Item>,
                        Item,
                    >(pull: Pull, push: Push) {
                        ::hydroflow::pusherator::pivot::Pivot::new(pull, push).run();
                    }
                    check_pivot_run(op_3v1, op_4v1);
                    context.schedule_subgraph(context.current_subgraph(), false);
                },
            );
            df
        }
    };
    df.run_available();
}
