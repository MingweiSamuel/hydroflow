#![feature(proc_macro_diagnostic, proc_macro_span)]
#![allow(clippy::explicit_auto_deref)]

use std::convert::identity;

use proc_macro2::{Ident, Literal, Punct, Span, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, AttrStyle, Expr, ExprLit, ItemConst, Lit, Member, Path, Token,
    Type,
};

#[proc_macro_attribute]
pub fn operator_docgen(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = parse_macro_input!(item as ItemConst);
    assert_eq!(
        identity::<Type>(parse_quote!(OperatorConstraints)),
        *item.ty
    );

    let Expr::Struct(expr_struct) = &*item.expr else {
        panic!("Unexpected non-struct expression.");
    };
    assert_eq!(
        identity::<Path>(parse_quote!(OperatorConstraints)),
        expr_struct.path
    );

    let name_field = expr_struct
        .fields
        .iter()
        .find(|&field_value| &identity::<Member>(parse_quote!(name)) == &field_value.member)
        .expect("Expected `name` field not found.");
    let Expr::Lit(ExprLit { lit: Lit::Str(op_name), .. }) = &name_field.expr else {
        panic!("Unexpected non-literal or non-str `name` field value.")
    };

    eprintln!("HELLO {}", op_name.value());

    for attr in item.attrs.iter() {
        if AttrStyle::Outer != attr.style {
            continue;
        }
        if identity::<Path>(parse_quote!(doc)) != attr.path {
            continue;
        }
        let tokens: Vec<_> = attr.tokens.clone().into_iter().collect();
        if 2 != tokens.len() {
            continue;
        }
        let TokenTree::Punct(punct) = &tokens[0] else {
            continue;
        };
        if '=' != punct.as_char() {
            continue;
        }
        let TokenTree::Literal(doc_lit_token) = &tokens[1] else {
            continue;
        };
        let doc_lit = Lit::new(doc_lit_token.clone());
        let Lit::Str(doc_lit_str) = doc_lit else {
            continue;
        };
        let doc_str = doc_lit_str.value();
        let doc_str = doc_str.strip_prefix(' ').unwrap_or(&*doc_str);
        eprintln!("DOC {}", doc_str);
    }
    // item.attrs.iter().filter(|&attr| AttrStyle::Outer == attr.style && )

    item.into_token_stream().into()
}
