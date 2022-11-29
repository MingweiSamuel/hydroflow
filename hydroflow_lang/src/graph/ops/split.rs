use super::{
    OperatorConstraints, OperatorWriteOutput, WriteContextArgs, WriteIteratorArgs, RANGE_1,
};

use quote::{quote, quote_spanned};

/// > 1 input stream of pair tuples `(A, B)`, 2 output streams
///
/// Takes the input stream of pairs and splits each one, delivers each item to
/// its corresponding side.
///
/// ```hydroflow
/// my_split = recv_iter(vec![("Hello", "Foo"), ("World", "Bar")]) -> split();
/// my_split[0] -> for_each(|x| println!("0: {}", x)); // Hello World
/// my_split[1] -> for_each(|x| println!("1: {}", x)); // Foo Bar
/// ```
#[hydroflow_internalmacro::operator_docgen]
pub const SPLIT: OperatorConstraints = OperatorConstraints {
    name: "split",
    hard_range_inn: RANGE_1,
    soft_range_inn: RANGE_1,
    hard_range_out: &(2..=2),
    soft_range_out: &(2..=2),
    ports_inn: None,
    ports_out: Some(&|| vec![quote!(0), quote!(1)]),
    num_args: 0,
    input_delaytype_fn: &|_| None,
    write_fn: &(|&WriteContextArgs { root, op_span, .. },
                 &WriteIteratorArgs {
                     ident,
                     outputs,
                     is_pull,
                     ..
                 }| {
        assert!(!is_pull);
        let output0 = &outputs[0];
        let output1 = &outputs[1];
        let write_iterator = quote_spanned! {op_span=>
            let #ident = #root::pusherator::split::Split::new(#output0, #output1);
        };
        OperatorWriteOutput {
            write_iterator,
            ..Default::default()
        }
    }),
};
