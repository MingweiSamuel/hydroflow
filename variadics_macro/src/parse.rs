use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Brace;
use syn::{
    braced, Attribute, Generics, Ident, ImplRestriction, Token, TraitItem, TypeParam,
    TypeParamBound, Visibility,
};

pub struct Variadic {
    pub variadic_token: Ident, // Should be "variadic"
    pub lt_token: Token![<],
    pub param: TypeParam,
    pub gt_token: Token![>],
}
impl Parse for Variadic {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variadic_token: Ident = input.parse()?;
        if variadic_token != "variadic" {
            return Err(syn::Error::new_spanned(
                variadic_token,
                "Expected `variadic`.",
            ));
        }
        let lt_token = input.parse()?;
        let param = input.parse()?;
        let gt_token = input.parse()?;

        Ok(Self {
            variadic_token,
            lt_token,
            param,
            gt_token,
        })
    }
}
impl ToTokens for Variadic {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.variadic_token.to_tokens(tokens);
        self.lt_token.to_tokens(tokens);
        self.param.to_tokens(tokens);
        self.gt_token.to_tokens(tokens);
    }
}

pub struct ItemVariadic {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafety: Option<Token![unsafe]>,
    pub auto_token: Option<Token![auto]>,
    pub restriction: Option<ImplRestriction>,
    pub variadic: Variadic,
    pub ident: Ident,
    pub generics: Generics,
    pub colon_token: Option<Token![:]>,
    pub supertraits: Punctuated<TypeParamBound, Token![+]>,
    pub brace_token: Brace,
    pub items: Vec<TraitItem>,
}
impl Parse for ItemVariadic {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let unsafety = input.parse()?;
        let auto_token = input.parse()?;
        let variadic = input.parse()?;
        let ident = input.parse()?;
        let mut generics: Generics = input.parse()?;

        let colon_token: Option<Token![:]> = input.parse()?;

        let mut supertraits = Punctuated::new();
        if colon_token.is_some() {
            loop {
                if input.peek(Token![where]) || input.peek(Brace) {
                    break;
                }
                supertraits.push_value(input.parse()?);
                if input.peek(Token![where]) || input.peek(Brace) {
                    break;
                }
                supertraits.push_punct(input.parse()?);
            }
        }

        generics.where_clause = input.parse()?;

        let content;
        let brace_token = braced!(content in input);
        // syn::attr::parsing::parse_inner(&content, &mut attrs)?;
        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse()?);
        }

        Ok(ItemVariadic {
            attrs,
            vis,
            unsafety,
            auto_token,
            restriction: None,
            variadic,
            ident,
            generics,
            colon_token,
            supertraits,
            brace_token,
            items,
        })
    }
}
impl ToTokens for ItemVariadic {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        // tokens.append_all(self.attrs.outer());
        for attr in self.attrs.iter() {
            attr.to_tokens(tokens);
        }
        self.vis.to_tokens(tokens);
        self.unsafety.to_tokens(tokens);
        self.auto_token.to_tokens(tokens);
        self.variadic.to_tokens(tokens);
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        if !self.supertraits.is_empty() {
            self.colon_token.unwrap_or_default().to_tokens(tokens);
            self.supertraits.to_tokens(tokens);
        }
        self.generics.where_clause.to_tokens(tokens);
        self.brace_token.surround(tokens, |tokens| {
            // tokens.append_all(self.attrs.inner());
            // tokens.append_all(&self.items);
            for item in self.items.iter() {
                item.to_tokens(tokens);
            }
        });
    }
}
