use super::{
    OperatorConstraints, OperatorWriteOutput, WriteContextArgs, WriteIteratorArgs, RANGE_ANY,
};

use proc_macro2::Span;
use quote::quote_spanned;

#[hydroflow_internalmacro::operator_docgen]
pub const NULL: OperatorConstraints = OperatorConstraints {
    name: "null",
    hard_range_inn: RANGE_ANY,
    soft_range_inn: RANGE_ANY,
    hard_range_out: RANGE_ANY,
    soft_range_out: RANGE_ANY,
    ports_inn: None,
    ports_out: None,
    num_args: 0,
    input_delaytype_fn: &|_| None,
    write_fn: &(|&WriteContextArgs { root, op_span, .. },
                 &WriteIteratorArgs {
                     ident,
                     inputs,
                     outputs,
                     is_pull,
                     ..
                 }| {
        let write_iterator = if is_pull {
            quote_spanned! {op_span=>
                (#(#inputs.for_each(std::mem::drop)),*);
                let #ident = std::iter::empty();
            }
        } else {
            quote_spanned! {op_span=>
                #[allow(clippy::let_unit_value)]
                let _ = (#(#outputs),*);
                let #ident = #root::pusherator::for_each::ForEach::new(std::mem::drop);
            }
        };
        OperatorWriteOutput {
            write_iterator,
            ..Default::default()
        }
    }),
    doc_example: &(|| {
        quote_spanned! {Span::call_site()=>
            recv_iter(vec!["hello", "world"]) -> null();
            null() -> for_each(|x: usize| unreachable!());
        }
    }),
};
