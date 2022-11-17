use super::{
    DelayType, OperatorConstraints, OperatorWriteOutput, WriteContextArgs, WriteIteratorArgs,
    RANGE_1,
};

use proc_macro2::Span;
use quote::quote_spanned;

#[hydroflow_internalmacro::operator_docgen]
pub const UNIQUE: OperatorConstraints = OperatorConstraints {
    name: "unique",
    hard_range_inn: RANGE_1,
    soft_range_inn: RANGE_1,
    hard_range_out: RANGE_1,
    soft_range_out: RANGE_1,
    ports_inn: None,
    ports_out: None,
    num_args: 0,
    input_delaytype_fn: &|_| Some(DelayType::Stratum),
    write_fn: &(|&WriteContextArgs { op_span, .. },
                 &WriteIteratorArgs {
                     ident,
                     inputs,
                     arguments,
                     is_pull,
                     ..
                 }| {
        assert!(is_pull);
        let input = &inputs[0];
        let write_iterator = quote_spanned! {op_span=>
            let #ident = #input.fold(HashSet::new(#arguments), |mut prev, nxt| {prev.insert(nxt); prev}).into_iter();
        };
        OperatorWriteOutput {
            write_iterator,
            ..Default::default()
        }
    }),
    doc_example: &(|| {
        quote_spanned! {Span::call_site()=>
            todo!();
        }
    }),
};
