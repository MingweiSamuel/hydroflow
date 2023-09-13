use quote::{quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_quote_spanned, Expr, LitInt, Pat, PatType, Token};

use super::{
    FlowProperties, FlowPropertyVal, OperatorCategory, OperatorConstraints, PortListSpec,
    WriteContextArgs, RANGE_0, RANGE_1,
};
use crate::diagnostic::{Diagnostic, Level};
use crate::graph::OperatorInstance;

// TODO(mingwei): update docs.

// TODO(mingwei): Preprocess rustdoc links in mdbook or in the `operator_docgen` macro.
/// > Arguments: A Rust closure, the first argument is a received item and the
/// > second argument is a variadic [`var_args!` tuple list](https://hydro-project.github.io/hydroflow/doc/hydroflow/macro.var_args.html)
/// > where each item name is an output port.
///
/// Takes the input stream and allows the user to determine which items to
/// deliver to any number of output streams.
///
/// > Note: Downstream operators may need explicit type annotations.
///
/// > Note: The [`Pusherator`](https://hydro-project.github.io/hydroflow/doc/pusherator/trait.Pusherator.html)
/// > trait is automatically imported to enable the [`.give(...)` method](https://hydro-project.github.io/hydroflow/doc/pusherator/trait.Pusherator.html#tymethod.give).
///
/// > Note: The closure has access to the [`context` object](surface_flows.md#the-context-object).
///
/// ```hydroflow
/// my_partition = source_iter(1..=100) -> partition(|v, [fzbz, fizz, buzz, vals]: [_; 4]|
///     match (*v % 3, *v % 5) {
///         (0, 0) => fzbz,
///         (0, _) => fizz,
///         (_, 0) => buzz,
///         (_, _) => rest,
///     }
/// );
/// my_partition[fzbz] -> for_each(|v| println!("{}: fizzbuzz", v));
/// my_partition[fizz] -> for_each(|v| println!("{}: fizz", v));
/// my_partition[buzz] -> for_each(|v| println!("{}: buzz", v));
/// my_partition[rest] -> for_each(|v| println!("{}", v));
/// ```
pub const PARTITION: OperatorConstraints = OperatorConstraints {
    name: "partition",
    categories: &[OperatorCategory::MultiOut],
    hard_range_inn: RANGE_1,
    soft_range_inn: RANGE_1,
    hard_range_out: &(2..),
    soft_range_out: &(2..),
    num_args: 1,
    persistence_args: RANGE_0,
    type_args: RANGE_0,
    is_external_input: false,
    ports_inn: None,
    ports_out: Some(|| PortListSpec::Variadic),
    properties: FlowProperties {
        deterministic: FlowPropertyVal::Preserve,
        monotonic: FlowPropertyVal::Preserve,
        inconsistency_tainted: false,
    },
    input_delaytype_fn: |_| None,
    flow_prop_fn: None,
    write_fn: |wc @ &WriteContextArgs {
                   op_span,
                   is_pull,
                   op_name,
                   op_inst: op_inst @ OperatorInstance { arguments, .. },
                   ..
               },
               diagnostics| {
        assert!(!is_pull);
        let func = arguments[0].clone();
        let Expr::Closure(mut func) = func else {
            diagnostics.push(Diagnostic::spanned(
                func.span(),
                Level::Error,
                "Argument must be a two-argument closure expression"),
            );
            return Err(());
        };
        if 2 != func.inputs.len() {
            diagnostics.push(Diagnostic::spanned(
                func.span(),
                Level::Error,
                &*format!(
                    "Closure provided to `{}(..)` must have two arguments: \
                    the first argument is the item, and the second argument lists ports, e.g. `var_args!(port_a, port_b, ..)`.",
                    op_name
                ),
            ));
            return Err(());
        }

        // Port idents specified in the closure's second argument.
        let mut arg2 = &mut func.inputs[1];
        let closure_idents = {
            if let Pat::Ident(pat_ident) = arg2 {
                arg2 = &mut *pat_ident
                    .subpat
                    .as_mut()
                    .expect("TODO(mingwei): EXPECTED SUBPAT")
                    .1;
            }
            let Pat::Slice(pat_slice) = arg2 else {
                panic!("TODO(mingwei) extpected slice pat");
            };
            pat_slice
                .elems
                .iter()
                .map(|pat| {
                    let Pat::Ident(pat_ident) = pat else {
                        panic!("TODO(mingwei) expected ident pat");
                    };
                    pat_ident.ident.clone()
                })
                .collect::<Vec<_>>()
        };
        let len = LitInt::new(&closure_idents.len().to_string(), op_span);
        let x: PatType = PatType {
            attrs: vec![],
            pat: Box::new(arg2.clone()),
            colon_token: parse_quote_spanned! {op_span=> : },
            ty: parse_quote_spanned! {op_span=> [usize; #len ] },
        };
        // let x: PatType = parse_quote_spanned! {op_span=>
        //     #arg2 : [usize; #len ]
        // };
        eprintln!("{:?}", x.to_token_stream().to_string());
        *arg2 = Pat::Type(x);
        eprintln!("HELLO WORLD");

        let idxs = (0..closure_idents.len())
            .map(|i| LitInt::new(&format!("{}_usize", i), op_span))
            .collect::<Vec<_>>();
        let arguments = parse_quote_spanned! { op_span=>
            |__item, var_args!( #( #closure_idents ),* )| {
                let __idx = (#func)(&__item, [ #( #idxs ),* ]);
                match __idx {
                    #(
                        #idxs => #closure_idents.give(__item),
                    )*
                    __unknown => panic!("Returned index {} is out-of-bounds.", __unknown),
                };
            }
        };

        (super::demux::DEMUX.write_fn)(
            &WriteContextArgs {
                op_inst: &OperatorInstance {
                    arguments,
                    ..op_inst.clone()
                },
                ..wc.clone()
            },
            diagnostics,
        )
    },
};
