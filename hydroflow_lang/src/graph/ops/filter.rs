use super::{
    OperatorConstraints, OperatorWriteOutput, WriteContextArgs, WriteIteratorArgs, RANGE_1,
};

use proc_macro2::Span;
use quote::quote_spanned;

#[hydroflow_internalmacro::operator_docgen]
pub const FILTER: OperatorConstraints = OperatorConstraints {
    name: "filter",
    hard_range_inn: RANGE_1,
    soft_range_inn: RANGE_1,
    hard_range_out: RANGE_1,
    soft_range_out: RANGE_1,
    ports_inn: None,
    ports_out: None,
    num_args: 1,
    input_delaytype_fn: &|_| None,
    write_fn: &(|&WriteContextArgs { root, op_span, .. },
                 &WriteIteratorArgs {
                     ident,
                     inputs,
                     outputs,
                     arguments,
                     is_pull,
                     ..
                 }| {
        let write_iterator = if is_pull {
            let input = &inputs[0];
            quote_spanned! {op_span=>
                let #ident = #input.filter(#arguments);
            }
        } else {
            let output = &outputs[0];
            quote_spanned! {op_span=>
                let #ident = #root::pusherator::filter::Filter::new(#arguments, #output);
            }
        };
        OperatorWriteOutput {
            write_iterator,
            ..Default::default()
        }
    }),
    doc_example: &(|| {
        quote_spanned! {Span::call_site()=>
            recv_iter(vec!["hello", "world"]) -> filter(|&x| x == "hello") -> for_each(|x| println!("{}", x));
        }
    }),
};
