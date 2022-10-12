use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

pub fn properties_codegen() -> TokenStream {
    let specs = super::OPERATORS.iter().map(|op_cons| {
        let mut op_pascal_name = op_cons.name.to_case(Case::Pascal);
        op_pascal_name.push_str("Props");
        let op_ident = Ident::new(&op_pascal_name, Span::call_site());
        quote! {
            pub struct #op_ident;
        }
    });
    quote! {
     #(
         #specs
     )*
    }
}
