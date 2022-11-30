use super::{
    OperatorConstraints, OperatorWriteOutput, WriteContextArgs, WriteIteratorArgs, RANGE_1,
};

use quote::{quote_spanned, ToTokens};
use syn::Arm;

/// > 1 input stream, *n* output streams
///
/// Takes the input stream and delivers a copy of each item to each output.
/// > Note: Downstream operators may need explicit type annotations.
///
/// ```hydroflow
/// my_match = recv_iter([Some(0), Some(1), Some(9), Some(10), None]) -> match();
/// my_match[Some(0) => ()] -> for_each(|()| println!("zero"));
/// my_match[Some(x) => x] -> for_each(|x| println!("{}", x));
/// my_match[None => ()] -> for_each(|()| println!("unknown"));
/// ```
///
/// ```hydroflow
/// my_match = recv_iter(0..100) -> match();
/// my_match[x if 0 == x % 15 => x] -> for_each(|x| println!("fizzbuzz"));
/// my_match[x if 0 == x %  3 => x] -> for_each(|x| println!("fizz"));
/// my_match[x if 0 == x %  5 => x] -> for_each(|x| println!("buzz"));
/// my_match[x => x]                -> for_each(|x| println!("{}", x));
/// ```
#[hydroflow_internalmacro::operator_docgen]
pub const MATCH: OperatorConstraints = OperatorConstraints {
    name: "match",
    hard_range_inn: RANGE_1,
    soft_range_inn: RANGE_1,
    hard_range_out: &(1..),
    soft_range_out: &(1..),
    ports_inn: None,
    ports_out: None,
    sort_ports_inn: true,
    sort_ports_out: false, // Do not sort, keep match arm order.
    num_args: 0,
    input_delaytype_fn: &|_| None,
    write_fn: &(|&WriteContextArgs { root, op_span, .. },
                 &WriteIteratorArgs {
                     ident,
                     inputs,
                     outputs,
                     output_ports,
                     is_pull,
                     ..
                 }| {
        let write_iterator = if !is_pull {
            let arms = output_ports
                .iter()
                .enumerate()
                .map(|(i, &port)| match port {
                    crate::graph::PortIndexValue::Tokens(tokens) => {
                        let mut arm = syn::parse2::<Arm>(tokens.clone()).unwrap(); // TODO MINGWEI ERROR HANDLING
                        let body = arm.body;
                        arm.body = syn::parse2((0..i).fold(
                            quote_spanned! {op_span=> #root::Either::Left(#[allow(unused_braces)] { #body }) },
                            |inner, _i| quote_spanned! {op_span=> #root::Either::Right(#inner) },
                        ))
                        .unwrap(); // TODO MINGWEI
                        arm
                    }
                    crate::graph::PortIndexValue::Elided(span) => {
                        // TODO MINGWEI USE ERROR STRUCT
                        span.unwrap()
                            .error("Must specify match arm as port.")
                            .emit();
                        panic!();
                    }
                });
            let switches = outputs
                .iter()
                .rev()
                .map(|i| i.to_token_stream())
                .fold(
                    quote_spanned!{op_span=> #root::pusherator::for_each::ForEach::new(std::mem::drop::<()>) },
                    |inner, next| quote_spanned!{op_span=> #root::pusherator::switch::Switch::new(#next, #inner) }
                );
            quote_spanned! {op_span=>
                let #ident = #root::pusherator::map::Map::new(|x| match x {
                    #( #arms, )*
                }, #switches);
            }
        } else {
            assert_eq!(1, outputs.len());
            let arm = match &output_ports[0] {
                crate::graph::PortIndexValue::Tokens(tokens) => tokens,
                crate::graph::PortIndexValue::Elided(span) => {
                    span.unwrap()
                        .error("Must specify match arm as port.")
                        .emit();
                    panic!()
                } // TODO MINGWEI
            };

            let input = &inputs[0];
            quote_spanned! {op_span=>
                let #ident = #input.map(|x| match x  { #arm });
            }
        };
        OperatorWriteOutput {
            write_iterator,
            ..Default::default()
        }
    }),
};
