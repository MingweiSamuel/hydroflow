#![deny(missing_docs)]
//! Derive macros for the `lattices` crate.

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, DeriveInput, Token};

fn root() -> TokenStream {
    let crate_name =
        proc_macro_crate::crate_name("lattices").expect("`lattices` not found in `Cargo.toml`");
    match crate_name {
        proc_macro_crate::FoundCrate::Itself => quote! { crate },
        proc_macro_crate::FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote! { :: #ident }
        }
    }
}

/// Derives `LatticeOrd` for a type that implements [`PartialOrd`](std::cmp::PartialOrd).
#[proc_macro_derive(LatticeOrd)]
pub fn derive_lattice_ord(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let root = root();
    let input = parse_macro_input!(input as DeriveInput);
    let input_ident = input.ident;
    let (_impl_generics_og, type_generics_og, _where_clause_og) = input.generics.split_for_impl();

    let mut rhs_ident = "Rhs".to_owned();
    while rhs_ident == input_ident.to_string()
        || input
            .generics
            .type_params()
            .map(|param| &param.ident)
            .chain(input.generics.const_params().map(|param| &param.ident))
            .any(|ident| rhs_ident == ident.to_string())
    {
        rhs_ident.insert_str(0, " ");
    }
    let rhs_ident = Ident::new(&*rhs_ident, Span::call_site());

    let mut generics_rhs = input.generics.clone();
    generics_rhs
        .params
        .push(parse_quote! { #rhs_ident: ?::std::marker::Sized });
    generics_rhs
        .make_where_clause()
        .predicates
        .push(parse_quote! { Self: ::std::cmp::PartialOrd< #rhs_ident > });
    let (impl_generics_rhs, _type_generics_rhs, where_clause_rhs) = generics_rhs.split_for_impl();

    quote! {
        impl #impl_generics_rhs #root::LatticeOrd< #rhs_ident > for #input_ident #type_generics_og #where_clause_rhs
        {}
    }.into()
}

// #[proc_macro_derive(LatticePartialOrd)]
// pub fn derive_lattice_partial_ord(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
// }
