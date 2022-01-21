#![feature(proc_macro_diagnostic)]

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::fold::Fold;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, parse_quote_spanned, AngleBracketedGenericArguments, FnArg,
    GenericArgument, GenericParam, Pat, PatType, Path, PathArguments, PathSegment, ReturnType,
    Token, TraitBound, TraitItemMethod, Type, TypePath, WherePredicate,
};

#[proc_macro]
pub fn ignore(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::new()
}

pub(crate) struct UnaryInput {
    pub output: Type,
    #[allow(dead_code)]
    pub arrow: Token![=>],
    pub body: TraitItemMethod,
}
impl Parse for UnaryInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let output = input.parse()?;
        let arrow = input.parse()?;
        let body = input.parse()?;
        Ok(Self {
            output,
            arrow,
            body,
        })
    }
}

pub(crate) fn type_as_single_segment(ty: &Type) -> Option<&PathSegment> {
    if let Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments,
        },
    }) = ty
    {
        if 1 == segments.len() {
            return Some(segments.last().unwrap());
        }
    }
    None
}

struct SelfToPrev;
impl Fold for SelfToPrev {
    fn fold_ident(&mut self, i: Ident) -> Ident {
        match &*i.to_string() {
            "Self" => Ident::new("Prev", i.span()),
            "Item" => Ident::new("ItemOut", i.span()),
            _ => i,
        }
    }
}

#[derive(Default)]
pub(crate) struct ExtractWhereClauseFnInputs {
    count: usize,
    pub old_input_types: Punctuated<Type, syn::token::Comma>,
    pub new_generic_params: Punctuated<Ident, syn::token::Comma>,
    pub item_in_generic_param: Option<Ident>,
}
impl Fold for ExtractWhereClauseFnInputs {
    fn fold_trait_bound(&mut self, trait_bound: TraitBound) -> TraitBound {
        let prev_item: Type = parse_quote! { Prev::ItemOut };

        let mut trait_bound = syn::fold::fold_trait_bound(self, trait_bound);
        if trait_bound.path.leading_colon.is_none() && 1 == trait_bound.path.segments.len() {
            if let PathSegment {
                ident: _,
                arguments: PathArguments::Parenthesized(fn_args),
            } = trait_bound.path.segments.last_mut().unwrap()
            {
                for input_arg in fn_args.inputs.iter_mut() {
                    let mut ident_str = "ItemIn".to_owned();
                    if 0 < self.count {
                        ident_str.push_str(&*self.count.to_string());
                    }
                    let ident = Ident::new(&*ident_str, input_arg.span());

                    let mut segments = Punctuated::new();
                    segments.push(PathSegment::from(ident.clone()));

                    self.old_input_types.push(input_arg.clone());

                    println!(
                        "{} := {} == {}",
                        &prev_item == input_arg,
                        prev_item.to_token_stream(),
                        input_arg.to_token_stream()
                    );
                    if &prev_item == input_arg {
                        self.item_in_generic_param = Some(ident.clone());
                    }

                    *input_arg = Type::Path(TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments,
                        },
                    });

                    self.new_generic_params.push(ident);
                    self.count += 1;
                }
            }
        }
        trait_bound
    }
}

struct SelfTo(pub Type);
impl Fold for SelfTo {
    fn fold_type(&mut self, i: Type) -> Type {
        // Box<dyn Iterator<Item = Prev::ItemOut>>
        let i = syn::fold::fold_type(self, i);
        if let Some(PathSegment {
            ident,
            arguments: PathArguments::None,
        }) = type_as_single_segment(&i)
        {
            if "Self" == &*ident.to_string() {
                return self.0.clone();
            }
        }
        i
    }
}

struct OutputTypeToNextItem<'a>(pub &'a Type);
impl<'a> Fold for OutputTypeToNextItem<'a> {
    fn fold_type(&mut self, i: Type) -> Type {
        println!(
            "{} := {} == {}",
            &i == self.0,
            self.0.to_token_stream(),
            i.to_token_stream()
        );
        if &i == self.0 {
            let span = i.span();
            parse_quote_spanned! { span=> Next::ItemIn }
        } else {
            syn::fold::fold_type(self, i)
        }
    }
}

#[proc_macro]
pub fn surface_unary(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let type_self: Type = parse_quote! { Self };
    let generic_arg_self: GenericArgument = parse_quote! { Self };
    /*
    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>;
    */

    let input = parse_macro_input!(input as UnaryInput);
    let output_type = input.output;
    let input_method = input.body;

    if let Some(body) = input_method.default {
        body.span().unwrap().error("Body must be omitted.").emit();
    }

    let return_type: &PathSegment = match &input_method.sig.output {
        ReturnType::Default => {
            input_method
                .sig
                .span()
                .unwrap()
                .error("Must have return type.")
                .emit();
            return Default::default();
        }
        ReturnType::Type(_rarrow, ty) => {
            if let Some(path_seg) = type_as_single_segment(&*ty) {
                path_seg
            } else {
                ty.span().unwrap().error("Must be simple identifier type.");
                return Default::default();
            }
        }
    };

    let idents = Idents::from_base(return_type.ident.clone());

    let return_generic_args = match &return_type.arguments {
        PathArguments::AngleBracketed(generic_args) => generic_args,
        _ => {
            return_type
                .span()
                .unwrap()
                .error("Return type must have generic arguments e.g. `<A, B>`.")
                .emit();
            return Default::default();
        }
    };

    // let base_generic_args = SelfToPrev
    //     .fold_angle_bracketed_generic_arguments(return_generic_args.clone())
    //     .args;
    let base_generic_args: Punctuated<GenericArgument, Token![,]> = return_generic_args
        .args
        .clone()
        .into_iter()
        .filter(|generic_arg| &generic_arg_self != generic_arg)
        .collect();

    let base_args: Punctuated<PatType, Token![,]> = input_method
        .sig
        .inputs
        .into_iter()
        .filter_map(|fn_arg| match fn_arg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat_type) => Some(pat_type),
        })
        .collect();

    let base_arg_idents: Punctuated<Ident, Token![,]> = base_args
        .iter()
        .filter_map(|pat_type| match &*pat_type.pat {
            Pat::Ident(pat_ident) => Some(pat_ident.ident.clone()),
            other => {
                other
                    .span()
                    .unwrap()
                    .error("Unexpected argument pattern.")
                    .emit();
                None
            }
        })
        .collect();

    let input_generic_params =
        input_method
            .sig
            .generics
            .params
            .iter()
            .filter_map(|generic_param| match generic_param {
                GenericParam::Type(type_param) => Some(type_param.ident.clone()),
                other => {
                    other
                        .span()
                        .unwrap()
                        .error("Unexpected non-type generic param.")
                        .emit();
                    None
                }
            });
    let impl_generic_params: Punctuated<Ident, Token![,]> = [format_ident!("Prev")]
        .into_iter()
        .chain(input_generic_params)
        .collect();
    let impl_where_clause_predicates = input_method
        .sig
        .generics
        .where_clause
        .map(|where_clause| SelfToPrev.fold_where_clause(where_clause))
        .map(|where_clause| where_clause.predicates);

    let mut where_clause_fn_input_extractor = ExtractWhereClauseFnInputs::default();
    let fn_input_where_clause_predicates =
        impl_where_clause_predicates.as_ref().map(|predicates| {
            predicates
                .iter()
                .cloned()
                .map(|where_predicate| {
                    where_clause_fn_input_extractor.fold_where_predicate(where_predicate)
                })
                .collect::<Punctuated<_, Token![,]>>()
        });

    let fn_input_old_input_types = where_clause_fn_input_extractor.old_input_types;
    let fn_input_new_generic_params = where_clause_fn_input_extractor.new_generic_params;
    let fn_input_item_in = where_clause_fn_input_extractor.item_in_generic_param;

    let quote_surface = quote_surface(
        &idents,
        &output_type,
        &base_generic_args,
        &impl_generic_params,
        impl_where_clause_predicates.as_ref(),
        &base_args,
        &base_arg_idents,
        &fn_input_old_input_types,
        return_generic_args,
    );
    let quote_pull = quote_pull(
        &idents,
        &base_generic_args,
        &base_args,
        &base_arg_idents,
        &impl_generic_params,
        &impl_where_clause_predicates,
    );
    let quote_push = quote_push(
        &idents,
        &output_type,
        &base_generic_args,
        &base_args,
        &base_arg_idents,
        &fn_input_new_generic_params,
        fn_input_item_in,
        fn_input_where_clause_predicates,
    );

    let expanded = quote! {
        #quote_surface
        #quote_pull
        #quote_push
    };
    expanded.into()
}

/// Create the **Surface struct.
pub(crate) fn quote_surface(
    idents: &Idents,
    output_type: &Type,
    base_generic_args: &Punctuated<syn::GenericArgument, Token![,]>,
    impl_generic_params: &Punctuated<Ident, Token![,]>,
    impl_where_clause_predicates: Option<&Punctuated<syn::WherePredicate, Token![,]>>,
    base_args: &Punctuated<PatType, Token![,]>,
    base_arg_idents: &Punctuated<Ident, Token![,]>,
    fn_input_old_input_types: &Punctuated<Type, Token![,]>,
    return_generic_args: &AngleBracketedGenericArguments,
) -> TokenStream {
    let Idents {
        ident_base: _,
        ident_surface,
        ident_pull_build,
        ident_push_surface_reversed,
        ident_push_build: _,
        ident_push_build_impl: _,
    } = idents;

    let pull_build_generic_args = SelfTo(parse_quote!(Prev::Build))
        .fold_angle_bracketed_generic_arguments(return_generic_args.clone());

    quote! {
        pub struct #ident_surface < Prev, #base_generic_args >
        where
            Prev: BaseSurface,
        {
            prev: Prev,
            #base_args
        }

        impl< #impl_generic_params > #ident_surface < Prev, #base_generic_args >
        where
            Prev: BaseSurface,
            #impl_where_clause_predicates
        {
            pub fn new(prev: Prev, #base_args) -> Self {
                Self { prev, #base_arg_idents }
            }
        }

        impl< #impl_generic_params > BaseSurface for #ident_surface < Prev, #base_generic_args >
        where
            Prev: BaseSurface,
            #impl_where_clause_predicates
        {
            type ItemOut = #output_type ;
        }

        impl< #impl_generic_params > PullSurface for #ident_surface < Prev, #base_generic_args >
        where
            Prev: PullSurface,
            #impl_where_clause_predicates
        {
            type InputHandoffs = Prev::InputHandoffs;

            type Connect = Prev::Connect;
            type Build = #ident_pull_build #pull_build_generic_args;

            fn into_parts(self) -> (Self::Connect, Self::Build) {
                let Self { prev, #base_arg_idents } = self;
                let (connect, build) = prev.into_parts();
                let build = #ident_pull_build ::new(build, #base_arg_idents );
                (connect, build)
            }
        }

        impl< #impl_generic_params > PushSurface for #ident_surface < Prev, #base_generic_args >
        where
            Prev: PushSurface,
            #impl_where_clause_predicates
        {
            type Output<Next>
            where
                Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
            = Prev::Output< #ident_push_surface_reversed <Next, #base_generic_args , #fn_input_old_input_types >>;

            fn reverse<Next>(self, next: Next) -> Self::Output<Next>
            where
                Next: PushSurfaceReversed<ItemIn = Self::ItemOut>,
            {
                let Self { prev, #base_arg_idents } = self;
                prev.reverse( #ident_push_surface_reversed ::new(next, #base_arg_idents ))
            }
        }
    }
}

pub(crate) fn quote_pull(
    idents: &Idents,
    base_generic_args: &Punctuated<syn::GenericArgument, Token![,]>,
    base_args: &Punctuated<PatType, Token![,]>,
    base_arg_idents: &Punctuated<Ident, Token![,]>,
    impl_generic_params: &Punctuated<Ident, Token![,]>,
    impl_where_clause_predicates: &Option<Punctuated<syn::WherePredicate, Token![,]>>,
) -> TokenStream {
    let ident_pull_build = &idents.ident_pull_build;

    quote! {
        pub struct #ident_pull_build < Prev, #base_generic_args >
        where
            Prev: PullBuild,
        {
            prev: Prev,
            #base_args
        }

        impl< #impl_generic_params > #ident_pull_build < Prev, #base_generic_args >
        where
            Prev: PullBuild,
            #impl_where_clause_predicates
        {
            pub fn new(prev: Prev, #base_args ) -> Self {
                Self { prev, #base_arg_idents }
            }
        }

        // #[allow(type_alias_bounds)]
        // type PullBuildImpl<'slf, 'hof, Prev, Func, Out>
        // where
        //     Prev: PullBuild,
        // = std::iter::FilterMap<Prev::Build<'slf, 'hof>, impl FnMut(Prev::ItemOut) -> Option<Out>>;

        // impl<Prev, Func, Out> PullBuildBase for FilterMapPullBuild<Prev, Func>
        // where
        //     Prev: PullBuild,
        //     Func: FnMut(Prev::ItemOut) -> Option<Out>,
        // {
        //     type ItemOut = Out;
        //     type Build<'slf, 'hof> = PullBuildImpl<'slf, 'hof, Prev, Func, Out>;
        // }

        // impl<Prev, Func, Out> PullBuild for FilterMapPullBuild<Prev, Func>
        // where
        //     Prev: PullBuild,
        //     Func: FnMut(Prev::ItemOut) -> Option<Out>,
        // {
        //     type InputHandoffs = Prev::InputHandoffs;

        //     fn build<'slf, 'hof>(
        //         &'slf mut self,
        //         handoffs: <Self::InputHandoffs as HandoffList>::RecvCtx<'hof>,
        //     ) -> Self::Build<'slf, 'hof> {
        //         self.prev.build(handoffs).filter_map(|x| (self.func)(x))
        //     }
        // }
    }
}

pub(crate) fn quote_push(
    idents: &Idents,
    output_type: &Type,
    base_generic_args: &Punctuated<syn::GenericArgument, Token![,]>,
    base_args: &Punctuated<PatType, Token![,]>,
    base_arg_idents: &Punctuated<Ident, Token![,]>,
    fn_input_new_generic_params: &Punctuated<Ident, Token![,]>,
    fn_input_item_in: Option<Ident>,
    fn_input_where_clause_predicates: Option<Punctuated<WherePredicate, Token![,]>>,
) -> TokenStream {
    let Idents {
        ident_base,
        ident_surface: _,
        ident_pull_build: _,
        ident_push_surface_reversed,
        ident_push_build,
        ident_push_build_impl,
    } = &idents;

    let updated_where_clause_predicates: Punctuated<WherePredicate, Token![,]> =
        fn_input_where_clause_predicates
            .into_iter()
            .flat_map(|punctuated| punctuated.clone().into_iter())
            .map(|where_predicate| {
                OutputTypeToNextItem(output_type).fold_where_predicate(where_predicate)
            })
            .collect();

    let item_in = fn_input_item_in
        .map(|ident| {
            Type::Path(TypePath {
                qself: None,
                path: ident.into(),
            })
        })
        .unwrap_or_else(|| parse_quote! { Next::ItemIn });

    quote! {
        pub struct #ident_push_surface_reversed <Next, #base_generic_args , #fn_input_new_generic_params >
        where
            Next: PushSurfaceReversed,
            #updated_where_clause_predicates
        {
            next: Next,
            #base_args,
            _phantom: std::marker::PhantomData<fn( #fn_input_new_generic_params )>,
        }

        impl<Next, #base_generic_args , #fn_input_new_generic_params > #ident_push_surface_reversed <Next, #base_generic_args , #fn_input_new_generic_params >
        where
            Next: PushSurfaceReversed,
            #updated_where_clause_predicates
        {
            pub fn new(next: Next, #base_args ) -> Self {
                Self {
                    next,
                    #base_arg_idents,
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        impl<Next, #base_generic_args , #fn_input_new_generic_params > PushSurfaceReversed for #ident_push_surface_reversed <Next, #base_generic_args , #fn_input_new_generic_params >
        where
            Next: PushSurfaceReversed,
            #updated_where_clause_predicates
        {
            type OutputHandoffs = Next::OutputHandoffs;

            type ItemIn = #item_in;

            type Connect = Next::Connect;
            type Build = #ident_push_build <Next::Build, #base_generic_args , #fn_input_new_generic_params >;

            fn into_parts(self) -> (Self::Connect, Self::Build) {
                let Self { next, #base_arg_idents, _phantom } = self;
                let (connect, build) = next.into_parts();
                let build = #ident_push_build ::new(build, #base_arg_idents );
                (connect, build)
            }
        }

        pub struct #ident_push_build <Next, #base_generic_args , #fn_input_new_generic_params > // TODO
        where
            Next: PushBuild,
            #updated_where_clause_predicates
        {
            next: Next,
            #base_args,
            _phantom: std::marker::PhantomData<fn( #fn_input_new_generic_params )>,
        }
        impl<Next, #base_generic_args , #fn_input_new_generic_params > FilterMapPushBuild<Next, #base_generic_args , #fn_input_new_generic_params >
        where
            Next: PushBuild,
            #updated_where_clause_predicates
        {
            pub fn new(next: Next, #base_args ) -> Self {
                Self {
                    next,
                    #base_arg_idents,
                    _phantom: std::marker::PhantomData,
                }
            }
        }

        #[allow(type_alias_bounds)]
        type #ident_push_build_impl <'slf, 'hof, Next, #base_generic_args , #fn_input_new_generic_params >
        where
            Next: PushBuild,
        = impl Pusherator<Item = #item_in >;

        impl<Next, #base_generic_args , #fn_input_new_generic_params > PushBuildBase for #ident_push_build <Next, #base_generic_args , #fn_input_new_generic_params >
        where
            Next: PushBuild,
            #updated_where_clause_predicates
        {
            type ItemIn = #item_in;
            type Build<'slf, 'hof> = #ident_push_build_impl <'slf, 'hof, Next, #base_generic_args , #fn_input_new_generic_params >;
        }

        impl<Next, #base_generic_args , #fn_input_new_generic_params > PushBuild for #ident_push_build <Next, #base_generic_args , #fn_input_new_generic_params >
        where
            Next: PushBuild,
            #updated_where_clause_predicates
        {
            type OutputHandoffs = Next::OutputHandoffs;

            fn build<'slf, 'hof>(
                &'slf mut self,
                handoffs: <Self::OutputHandoffs as HandoffList>::SendCtx<'hof>,
            ) -> Self::Build<'slf, 'hof> {
                hydroflow::compiled::filter_map:: // TODO!!!
                #ident_base ::new(|x| (self.f)(x), self.next.build(handoffs)) // TODO!!!
            }
        }
    }
}

pub(crate) struct Idents {
    pub ident_base: Ident,
    pub ident_surface: Ident,
    pub ident_pull_build: Ident,
    pub ident_push_surface_reversed: Ident,
    pub ident_push_build: Ident,
    pub ident_push_build_impl: Ident,
}
impl Idents {
    pub fn from_base(ident_base: Ident) -> Self {
        let ident_surface = format_ident!("{}Surface", ident_base);
        let ident_pull_build = format_ident!("{}PullBuild", ident_base);
        let ident_push_surface_reversed = format_ident!("{}PushSurfaceReversed", ident_base);
        let ident_push_build = format_ident!("{}PushBuild", ident_base);
        let ident_push_build_impl = format_ident!("{}PushBuildImpl", ident_base);
        Self {
            ident_base,
            ident_surface,
            ident_pull_build,
            ident_push_surface_reversed,
            ident_push_build,
            ident_push_build_impl,
        }
    }
}
