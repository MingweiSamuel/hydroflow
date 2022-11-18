use super::{
    OperatorConstraints, OperatorWriteOutput, WriteContextArgs, WriteIteratorArgs, RANGE_1,
    RANGE_ANY,
};

use proc_macro2::Span;
use quote::{quote_spanned, ToTokens};

#[hydroflow_internalmacro::operator_docgen]
pub const MERGE: OperatorConstraints = OperatorConstraints {
    name: "merge",
    hard_range_inn: RANGE_ANY,
    soft_range_inn: &(2..),
    hard_range_out: RANGE_1,
    soft_range_out: RANGE_1,
    ports_inn: None,
    ports_out: None,
    num_args: 0,
    input_delaytype_fn: &|_| None,
    write_fn: &(|&WriteContextArgs { op_span, .. },
                 &WriteIteratorArgs {
                     ident,
                     inputs,
                     outputs,
                     is_pull,
                     ..
                 }| {
        let write_iterator = if is_pull {
            let chains = inputs
                .iter()
                .map(|i| i.to_token_stream())
                .reduce(|a, b| quote_spanned! {op_span=> #a.chain(#b) })
                .unwrap_or_else(|| quote_spanned! {op_span=> std::iter::empty() });
            quote_spanned! {op_span=>
                let #ident = #chains;
            }
        } else {
            assert_eq!(1, outputs.len());
            let output = &outputs[0];
            quote_spanned! {op_span=>
                let #ident = #output;
            }
        };
        OperatorWriteOutput {
            write_iterator,
            ..Default::default()
        }
    }),
};
