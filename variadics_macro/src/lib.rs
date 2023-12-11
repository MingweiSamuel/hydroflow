use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, parse_quote_spanned, Ident};

use crate::parse::ItemVariadic;

mod parse;

fn sealed_root() -> proc_macro2::TokenStream {
    let hydroflow_crate = proc_macro_crate::crate_name("sealed")
        .expect("`sealed` should be present in `Cargo.toml` `dependencies`.");
    match hydroflow_crate {
        proc_macro_crate::FoundCrate::Itself => {
            quote! { ::sealed }
        }
        proc_macro_crate::FoundCrate::Name(name) => {
            let ident: Ident = Ident::new(&name, Span::call_site());
            quote! { ::#ident }
        }
    }
}

#[proc_macro]
pub fn variadic_trait2(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let sealed_root = sealed_root();

    let item = parse_macro_input!(tokens as ItemVariadic);
    let ItemVariadic {
        attrs,
        vis,
        unsafety,
        auto_token,
        restriction: _,
        variadic,
        ident,
        generics,
        colon_token,
        supertraits,
        brace_token,
        items,
    } = item;

    let var_item = variadic.param.ident;

    // let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    // let mut generics_with_rest = generics.clone();
    // generics_with_rest.params.push(parse_quote! { __Rest: #ident });
    // let (impl_generics_wr, _ty_generics_wr, where_clause_wr) = generics_with_rest.split_for_impl();

    quote! {
        #( #attrs )*
        #[#sealed_root::sealed]
        #vis #unsafety #auto_token trait #ident #colon_token #supertraits #generics {

        }
        #[#sealed_root::sealed]
        impl<#var_item, __Rest: #ident> #ident for (#var_item, __Rest) {

        }
        #[#sealed_root::sealed]
        impl #ident for () {

        }
    }
    .into()
}
