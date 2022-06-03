use proc_macro::Ident;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{parenthesized, parse_macro_input, Expr, ExprCall, ExprPath, Token};

struct Pipeline {
    pub elems: Punctuated<Operator, Token![>>]>,
}
impl Parse for Pipeline {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut elems = Punctuated::new();

        while !input.is_empty() {
            let first = input.parse()?;
            elems.push_value(first);
            if input.is_empty() {
                break;
            }
            let punct = input.parse()?;
            elems.push_punct(punct);
        }

        Ok(Self { elems })
    }
}
impl ToTokens for Pipeline {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.elems.to_tokens(tokens);
    }
}

struct Operator {
    pub path: ExprPath,
    pub paren_token: Paren,
    pub args: Punctuated<Expr, Token![,]>,
}
impl Parse for Operator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.parse()?;

        let content;
        let paren_token = parenthesized!(content in input);
        let mut args = Punctuated::new();

        while !content.is_empty() {
            let first = content.parse()?;
            args.push_value(first);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            args.push_punct(punct);
        }

        Ok(Self {
            path,
            paren_token,
            args,
        })
    }
}
impl ToTokens for Operator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        todo!()
    }
}

#[proc_macro]
pub fn hydroflow_parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Pipeline);
    // input.into_token_stream().into()
    quote! { println!("hello world"); }.into()
}
