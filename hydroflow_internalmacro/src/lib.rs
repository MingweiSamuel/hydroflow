#![feature(proc_macro_diagnostic, proc_macro_span)]
#![allow(clippy::explicit_auto_deref)]

use proc_macro2::{Ident, Literal, Span};
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn operator_docgen(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    item
}
