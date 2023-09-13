use hydroflow::hydroflow_syntax;
use multiplatform_test::multiplatform_test;

#[test]
pub fn test_partition_fizzbuzz() {
    let mut df = {
        #[allow(unused_qualifications)]
        {
            use ::hydroflow::{var_expr, var_args};
            let mut df = ::hydroflow::scheduled::graph::Hydroflow::new();
            df.__assign_meta_graph(
                "{\"nodes\":[{\"value\":null,\"version\":0},{\"value\":{\"Operator\":\"source_iter(1 ..= 100)\"},\"version\":1},{\"value\":{\"Operator\":\"partition(| v, [fzbz, fizz, buzz, vals] | match(v % 3, v % 5)\\n{ (0, 0) => fzbz, (0, _) => fizz, (_, 0) => buzz, (_, _) => vals, })\"},\"version\":1},{\"value\":{\"Operator\":\"for_each(| _ | println! (\\\"fizzbuzz\\\"))\"},\"version\":1},{\"value\":{\"Operator\":\"for_each(| _ | println! (\\\"fizz\\\"))\"},\"version\":1},{\"value\":{\"Operator\":\"for_each(| _ | println! (\\\"buzz\\\"))\"},\"version\":1},{\"value\":{\"Operator\":\"for_each(| x | println! (\\\"{}\\\", x))\"},\"version\":1}],\"graph\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":3,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":4,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":5,\"version\":1}],\"version\":1},{\"value\":[{\"idx\":2,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1}],\"ports\":[{\"value\":null,\"version\":0},{\"value\":[\"Elided\",\"Elided\"],\"version\":1},{\"value\":[{\"Path\":\"fzbz\"},\"Elided\"],\"version\":1},{\"value\":[{\"Path\":\"fizz\"},\"Elided\"],\"version\":1},{\"value\":[{\"Path\":\"buzz\"},\"Elided\"],\"version\":1},{\"value\":[{\"Path\":\"vals\"},\"Elided\"],\"version\":1}],\"node_subgraph\":[{\"value\":null,\"version\":0},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1},{\"value\":{\"idx\":1,\"version\":1},\"version\":1}],\"subgraph_nodes\":[{\"value\":null,\"version\":0},{\"value\":[{\"idx\":1,\"version\":1},{\"idx\":2,\"version\":1},{\"idx\":3,\"version\":1},{\"idx\":4,\"version\":1},{\"idx\":5,\"version\":1},{\"idx\":6,\"version\":1}],\"version\":1}],\"subgraph_stratum\":[{\"value\":null,\"version\":0},{\"value\":0,\"version\":1}],\"node_varnames\":[{\"value\":null,\"version\":0},{\"value\":\"my_partition\",\"version\":1},{\"value\":\"my_partition\",\"version\":1}],\"flow_props\":[{\"value\":null,\"version\":0}]}",
            );
            df.__assign_diagnostics("[]");
            let mut sg_1v1_node_1v1_iter = {
                #[inline(always)]
                fn check_iter<IntoIter: ::std::iter::IntoIterator<Item = Item>, Item>(
                    into_iter: IntoIter,
                ) -> impl ::std::iter::Iterator<Item = Item> {
                    ::std::iter::IntoIterator::into_iter(into_iter)
                }
                check_iter(1..=100)
            };
            df.add_subgraph_stratified(
                "Subgraph GraphSubgraphId(1v1)",
                0,
                (),
                (),
                move |context, (), ()| {
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
                    let op_6v1 = ::hydroflow::pusherator::for_each::ForEach::new(|x| {
                        (print!("{0}\n", x));
                    });
                    let op_6v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_6v1__for_each__loc_unknown_start_0_0_end_0_0<
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
                        op_6v1__for_each__loc_unknown_start_0_0_end_0_0(op_6v1)
                    };
                    let op_5v1 = ::hydroflow::pusherator::for_each::ForEach::new(|_| {
                        (print!("buzz\n"));
                    });
                    let op_5v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_5v1__for_each__loc_unknown_start_0_0_end_0_0<
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
                        op_5v1__for_each__loc_unknown_start_0_0_end_0_0(op_5v1)
                    };
                    let op_4v1 = ::hydroflow::pusherator::for_each::ForEach::new(|_| {
                        (print!("fizz\n"));
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
                    let op_3v1 = ::hydroflow::pusherator::for_each::ForEach::new(|_| {
                        (print!("fizzbuzz\n"));
                    });
                    let op_3v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_3v1__for_each__loc_unknown_start_0_0_end_0_0<
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
                        op_3v1__for_each__loc_unknown_start_0_0_end_0_0(op_3v1)
                    };
                    let op_2v1 = {
                        #[allow(unused_imports)]
                        use ::hydroflow::pusherator::Pusherator;
                        ::hydroflow::pusherator::demux::Demux::new(
                            |__item, (fzbz, (fizz, (buzz, (vals, ()))))| {
                                let __idx = (|v, [fzbz, fizz, buzz, vals]: [usize; 4]| match (
                                    v % 3,
                                    v % 5,
                                ) {
                                    (0, 0) => fzbz,
                                    (0, _) => fizz,
                                    (_, 0) => buzz,
                                    (_, _) => vals,
                                })(&__item, [0_usize, 1_usize, 2_usize, 3_usize]);
                                ([fzbz, fizz, buzz, vals])[__idx].give(__item)
                            },
                            (op_3v1, (op_4v1, (op_5v1, (op_6v1, ())))),
                        )
                    };
                    let op_2v1 = {
                        #[allow(non_snake_case)]
                        #[inline(always)]
                        pub fn op_2v1__partition__loc_unknown_start_0_0_end_0_0<
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
                        op_2v1__partition__loc_unknown_start_0_0_end_0_0(op_2v1)
                    };
                    #[inline(always)]
                    fn check_pivot_run<
                        Pull: ::std::iter::Iterator<Item = Item>,
                        Push: ::hydroflow::pusherator::Pusherator<Item = Item>,
                        Item,
                    >(pull: Pull, push: Push) {
                        ::hydroflow::pusherator::pivot::Pivot::new(pull, push).run();
                    }
                    check_pivot_run(op_1v1, op_2v1);
                },
            );
            df
        }
    };
    df.run_available();
}
