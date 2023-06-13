use quote::quote_spanned;

use super::{
    DelayType, FlowProperties, FlowPropertyVal, OperatorCategory, OperatorConstraints,
    OperatorWriteOutput, Persistence, WriteContextArgs, RANGE_0, RANGE_1,
};
use crate::graph::{OpInstGenerics, OperatorInstance};

/// > 1 input stream, 1 output stream
///
/// > Arguments: a closure which itself takes two arguments:
/// an 'accumulator', and an element. The accumulator is an `&mut` reference which can be modified
/// with the item. The closure does not return any value.
///
/// Akin to Rust's built-in reduce operator. Combines all elements together into an accumulatted
/// value by applying the closure, returning the final result. Note the closure supplied is
/// different from the one in [`std::iter::Iterator::reduce`], which takes in two owned values and
/// returns a combined value.
///
/// > Note: The closure has access to the [`context` object](surface_flows.md#the-context-object).
///
/// ```hydroflow
/// // should print 120 (i.e., 1*2*3*4*5)
/// source_iter([1,2,3,4,5])
///     -> reduce(|accum, elem| {
///         accum *= elem;
///     })
///     -> for_each(|e| println!("{}", e));
/// ```
pub const REDUCE: OperatorConstraints = OperatorConstraints {
    name: "reduce",
    categories: &[OperatorCategory::Fold],
    hard_range_inn: RANGE_1,
    soft_range_inn: RANGE_1,
    hard_range_out: RANGE_1,
    soft_range_out: RANGE_1,
    num_args: 1,
    persistence_args: &(0..=1),
    type_args: RANGE_0,
    is_external_input: false,
    ports_inn: None,
    ports_out: None,
    properties: FlowProperties {
        deterministic: FlowPropertyVal::DependsOnArgs,
        monotonic: FlowPropertyVal::DependsOnArgs,
        inconsistency_tainted: false,
    },
    input_delaytype_fn: |_| Some(DelayType::Stratum),
    write_fn: |wc @ &WriteContextArgs {
                   context,
                   hydroflow,
                   op_span,
                   ident,
                   inputs,
                   is_pull,
                   op_inst:
                       OperatorInstance {
                           arguments,
                           generics:
                               OpInstGenerics {
                                   persistence_args, ..
                               },
                           ..
                       },
                   ..
               },
               _| {
        assert!(is_pull);

        let persistence = match persistence_args[..] {
            [] => Persistence::Static,
            [a] => a,
            _ => unreachable!(),
        };

        let input = &inputs[0];
        let func = &arguments[0];
        let reducedata_ident = wc.make_ident("reducedata_ident");

        let (write_prologue, write_iterator, write_iterator_after) = match persistence {
            Persistence::Tick => (
                Default::default(),
                quote_spanned! {op_span=>
                    let #ident = #input.reduce(|mut acc, item| {
                        (#func)(&mut acc, item);
                        acc
                }).into_iter();
                },
                Default::default(),
            ),
            Persistence::Static => (
                quote_spanned! {op_span=>
                    let #reducedata_ident = #hydroflow.add_state(
                        ::std::cell::Cell::new(::std::option::Option::None)
                    );
                },
                quote_spanned! {op_span=>
                    let #ident = {
                        let opt = #context.state_ref(#reducedata_ident).take();
                        let opt = match opt {
                            Some(accum) => Some(#input.fold(accum, |mut acc, item| {
                                (#func)(&mut acc, item);
                                acc
                            })),
                            None => #input.reduce(|mut acc, item| {
                                (#func)(&mut acc, item);
                                acc
                            }),
                        };
                        #context.state_ref(#reducedata_ident).set(::std::clone::Clone::clone(&opt));
                        opt.into_iter()
                    };
                },
                quote_spanned! {op_span=>
                    #context.schedule_subgraph(#context.current_subgraph(), false);
                },
            ),
        };

        Ok(OperatorWriteOutput {
            write_prologue,
            write_iterator,
            write_iterator_after,
        })
    },
};
