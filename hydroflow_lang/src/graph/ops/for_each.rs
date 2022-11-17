use super::{
    OperatorConstraints, OperatorWriteOutput, WriteContextArgs, WriteIteratorArgs, RANGE_0, RANGE_1,
};

use proc_macro2::Span;
use quote::quote_spanned;

#[hydroflow_internalmacro::operator_docgen]
pub const FOR_EACH: OperatorConstraints = OperatorConstraints {
    name: "for_each",
    hard_range_inn: RANGE_1,
    soft_range_inn: RANGE_1,
    hard_range_out: RANGE_0,
    soft_range_out: RANGE_0,
    ports_inn: None,
    ports_out: None,
    num_args: 1,
    input_delaytype_fn: &|_| None,
    write_fn: &(|&WriteContextArgs { root, op_span, .. },
                 &WriteIteratorArgs {
                     ident, arguments, ..
                 }| {
        let write_iterator = quote_spanned! {op_span=>
            let #ident = #root::pusherator::for_each::ForEach::new(#arguments);
        };
        OperatorWriteOutput {
            write_iterator,
            ..Default::default()
        }
    }),
    doc_example: &(|| {
        quote_spanned! {Span::call_site()=>
            recv_iter(vec!["Hello", "World"]) -> for_each(|x| println!("{}", x));
        }
    }),
};
