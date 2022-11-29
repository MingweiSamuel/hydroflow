use super::{
    OperatorConstraints, OperatorWriteOutput, WriteContextArgs, WriteIteratorArgs, RANGE_1,
};

use quote::{quote, quote_spanned};

/// > 1 input stream of [`Either<A, B>`](either::Either), 2 output streams
///
/// Takes the input stream of `Either`s and delivers each item to its
/// corresponding side.
///
/// ```hydroflow
/// my_switch = recv_iter(vec![
///     hydroflow::Either::Left("Hello"),
///     hydroflow::Either::Right("World"),
/// ]) -> switch();
/// my_switch[left]  -> for_each(|x| println!("left: {}", x));  // left: Hello
/// my_switch[right] -> for_each(|x| println!("right: {}", x)); // right: World
/// ```
#[hydroflow_internalmacro::operator_docgen]
pub const SWITCH: OperatorConstraints = OperatorConstraints {
    name: "switch",
    hard_range_inn: RANGE_1,
    soft_range_inn: RANGE_1,
    hard_range_out: &(2..=2),
    soft_range_out: &(2..=2),
    ports_inn: None,
    ports_out: Some(&|| vec![quote!(left), quote!(right)]),
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
            let #ident = #root::pusherator::switch::Switch::new(#output0, #output1);
        };
        OperatorWriteOutput {
            write_iterator,
            ..Default::default()
        }
    }),
};
