use std::path::PathBuf;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Ident};

use crate::diagnostic::{Diagnostic, Level};
use crate::graph::build_hfcode;
use crate::parse::HfCode;

/// Returns the path to the `hydroflow` crate.
pub fn hydroflow_root() -> TokenStream {
    use std::env::{var as env_var, VarError};

    let hydroflow_crate = proc_macro_crate::crate_name("hydroflow")
        .expect("`hydroflow` should be present in `Cargo.toml`");
    match hydroflow_crate {
        proc_macro_crate::FoundCrate::Itself => {
            if Err(VarError::NotPresent) == env_var("CARGO_BIN_NAME")
                && Err(VarError::NotPresent) != env_var("CARGO_PRIMARY_PACKAGE")
                && Ok("hydroflow") == env_var("CARGO_CRATE_NAME").as_deref()
            {
                // In the crate itself, including unit tests.
                quote! { crate }
            } else {
                // In an integration test, example, bench, etc.
                quote! { ::hydroflow }
            }
        }
        proc_macro_crate::FoundCrate::Name(name) => {
            let ident: Ident = Ident::new(&name, Span::call_site());
            quote! { ::#ident }
        }
    }
}

pub fn build_emit_hydroflow(
    input: HfCode,
    min_diagnostic_level: Option<Level>,
    macro_invocation_path: PathBuf,
) -> TokenStream {
    let (graph_code_opt, diagnostics) = build_hfcode(input, macro_invocation_path);
    emit_hydroflow(graph_code_opt, diagnostics, min_diagnostic_level)
}

fn emit_hydroflow(
    graph_code_opt: Option<(crate::graph::HydroflowGraph, TokenStream)>,
    diagnostics: Vec<Diagnostic>,
    min_diagnostic_level: Option<Level>,
) -> TokenStream {
    let root = hydroflow_root();

    let tokens = graph_code_opt
        .map(|(_graph, code)| code)
        .unwrap_or_else(|| quote! { #root::scheduled::graph::Hydroflow::new() });

    let diagnostics = diagnostics
        .iter()
        .filter(|diag: &&Diagnostic| Some(diag.level) <= min_diagnostic_level);

    #[cfg(feature = "diagnostics")]
    {
        diagnostics.for_each(Diagnostic::emit);
        tokens.into()
    }

    #[cfg(not(feature = "diagnostics"))]
    {
        let diagnostics = diagnostics.map(Diagnostic::to_tokens);
        quote! {
            {
                #(
                    #diagnostics
                )*
                #tokens
            }
        }
        .into()
    }
}
