use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::discouraged::Speculative;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{parenthesized, parse_macro_input, Expr, ExprPath, Ident, Token};

struct HfCode {
    statements: Punctuated<HfStatement, Token![;]>,
}
impl Parse for HfCode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let statements = input.parse_terminated(HfStatement::parse)?;
        Ok(HfCode { statements })
    }
}
impl ToTokens for HfCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.statements.to_tokens(tokens)
    }
}

struct HfStatement {
    name: Option<Ident>,
    equals: Option<Token![=]>,
    value: Pipeline,
}
impl Parse for HfStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut equals = None;
        if input.peek2(Token![=]) {
            name = Some(input.parse()?);
            equals = Some(input.parse()?);
        }
        let value = input.parse()?;

        Ok(HfStatement {
            name,
            equals,
            value,
        })
    }
}
impl ToTokens for HfStatement {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.name.to_tokens(tokens);
        self.equals.to_tokens(tokens);
        self.value.to_tokens(tokens);
    }
}

enum Pipeline {
    Chain(ChainPipeline),
    ExprPath(ExprPath),
    Operator(Operator),
}
impl Parse for Pipeline {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Paren) {
            Ok(Self::Chain(input.parse()?))
            // Ok(Self::Paren(expr_paren))
        } else {
            let fork = input.fork();
            let expr_path = fork.parse()?;
            if fork.peek(Paren) {
                Ok(Self::Operator(input.parse()?))
            } else {
                input.advance_to(&fork);
                Ok(Self::ExprPath(expr_path))
            }
        }
    }
}
impl ToTokens for Pipeline {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Pipeline::Chain(x) => x.to_tokens(tokens),
            Pipeline::ExprPath(x) => x.to_tokens(tokens),
            Pipeline::Operator(x) => x.to_tokens(tokens),
        }
    }
}

struct ChainPipeline {
    pub paren_token: Paren,
    pub elems: Punctuated<Pipeline, Token![->]>,
}
impl Parse for ChainPipeline {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let paren_token = parenthesized!(content in input);
        let mut elems = Punctuated::new();

        while !content.is_empty() {
            let first = content.parse()?;
            elems.push_value(first);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            elems.push_punct(punct);
        }

        Ok(Self { paren_token, elems })
    }
}
impl ToTokens for ChainPipeline {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren_token.surround(tokens, |tokens| {
            self.elems.to_tokens(tokens);
        });
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
        self.path.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.args.to_tokens(tokens);
        });
    }
}

#[proc_macro]
pub fn hydroflow_parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as HfCode);
    // input.into_token_stream().into()
    quote! { println!("hello world"); }.into()
}
