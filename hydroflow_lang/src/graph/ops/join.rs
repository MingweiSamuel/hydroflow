use super::{
    OperatorConstraints, OperatorWriteOutput, WriteContextArgs, WriteIteratorArgs, RANGE_1,
};

use proc_macro2::Span;
use quote::quote_spanned;
use syn::parse_quote;

#[hydroflow_internalmacro::operator_docgen]
pub const JOIN: OperatorConstraints = OperatorConstraints {
    name: "join",
    hard_range_inn: &(2..=2),
    soft_range_inn: &(2..=2),
    hard_range_out: RANGE_1,
    soft_range_out: RANGE_1,
    ports_inn: Some(&(|| parse_quote! { 0, 1 })),
    ports_out: None,
    num_args: 0,
    input_delaytype_fn: &|_| None,
    write_fn: &(|wc @ &WriteContextArgs { root, op_span, .. },
                 &WriteIteratorArgs { ident, inputs, .. }| {
        let joindata_ident = wc.make_ident("joindata");
        let write_prologue = quote_spanned! {op_span=>
            let mut #joindata_ident = Default::default();
        };

        let lhs = &inputs[0];
        let rhs = &inputs[1];
        let write_iterator = quote_spanned! {op_span=>
            let #ident = #root::compiled::pull::SymmetricHashJoin::new(#lhs, #rhs, &mut #joindata_ident);
        };

        OperatorWriteOutput {
            write_prologue,
            write_iterator,
            ..Default::default()
        }
    }),
};
