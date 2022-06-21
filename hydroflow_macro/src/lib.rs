#![feature(proc_macro_diagnostic, proc_macro_span)]

use std::collections::HashMap;

use proc_macro2::{Literal, Span};
use quote::{quote, ToTokens};
use slotmap::{new_key_type, Key, SecondaryMap, SlotMap};
use syn::punctuated::Pair;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Ident, LitInt};

mod parse;
use parse::{HfCode, HfStatement, Operator, Pipeline};

#[proc_macro]
pub fn hydroflow_parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as HfCode);
    // // input.into_token_stream().into()

    let mut graph = Graph::from_hfcode(input).unwrap(/* TODO(mingwei) */);
    graph.validate_operators();
    graph.identify_subgraphs();

    // let debug = format!("{:#?}", graph);
    // let mut debug = String::new();
    // graph.write_graph(&mut debug).unwrap();
    let debug = graph.mermaid_string();

    let lit = Literal::string(&*debug);

    quote! { println!("{}", #lit); }.into()
}

new_key_type! { struct OperatorId; }
new_key_type! { struct SubgraphId; }

#[derive(Debug, Default)]
struct Graph {
    operators: SlotMap<OperatorId, OpInfo>,
    names: HashMap<Ident, Ports>,
    subgraphs: SlotMap<SubgraphId, Vec<OperatorId>>,
}
impl Graph {
    pub fn from_hfcode(input: HfCode) -> Result<Self, ()> {
        let mut graph = Self::default();

        for stmt in input.statements {
            graph.add_statement(stmt);
        }

        Ok(graph)
    }

    fn add_statement(&mut self, stmt: HfStatement) {
        match stmt {
            HfStatement::Named(named) => {
                let ports = self.add_pipeline(named.pipeline);
                // if let Some((old_name, _)) = self.names.remove_entry(&named.name) {
                //     old_name.span().unwrap().warning(format!("`{}` is shadowed"))
                // }
                self.names.insert(named.name, ports);
            }
            HfStatement::Pipeline(pipeline) => {
                self.add_pipeline(pipeline);
            }
        }
    }

    fn add_pipeline(&mut self, pipeline: Pipeline) -> Ports {
        match pipeline {
            Pipeline::Chain(chain_pipeline) => {
                // Handle chain pipelines as follows:
                let output = chain_pipeline
                    .elems
                    .into_pairs()
                    .map(Pair::into_tuple)
                    // 1. Resolve all the nested pipelines in first stage (collect into Vec before continuing, for ownership).
                    .map(|(pipeline, arrow)| (self.add_pipeline(pipeline), arrow))
                    .collect::<Vec<_>>()
                    .into_iter()
                    // 2. Iterate each element in pairs via `.reduce()` and combine them into the next pipeline.
                    // Essentially, treats the arrows as a left-associative binary operation (not that the direction really matters).
                    // `curr_ports: Ports` tracks the current input/output operators/ports in the graph.
                    .reduce(|(curr_ports, curr_arrow), (next_ports, next_arrow)| {
                        let curr_arrow =
                            curr_arrow.expect("Cannot have missing intermediate arrow");

                        if let (Some(out), Some(inn)) = (curr_ports.out, next_ports.inn) {
                            let src_port = curr_arrow.src.map(|x| x.index).unwrap_or_else(|| {
                                LitInt::new(
                                    &*self.operators[out].succs.len().to_string(),
                                    curr_arrow.arrow.span(),
                                )
                            });
                            let dst_port = curr_arrow.dst.map(|x| x.index).unwrap_or_else(|| {
                                LitInt::new(
                                    &*self.operators[inn].preds.len().to_string(),
                                    curr_arrow.arrow.span(),
                                )
                            });

                            {
                                /// Helper to emit conflicts when a port is overwritten.
                                fn emit_conflict(inout: &str, old: &LitInt, new: &LitInt) {
                                    old.span()
                                        .unwrap()
                                        .error(format!(
                                            "{} connection conflicts with below ({})",
                                            inout,
                                            PrettySpan(new.span()),
                                        ))
                                        .emit();
                                    new.span()
                                        .unwrap()
                                        .error(format!(
                                            "{} connection conflicts with above ({})",
                                            inout,
                                            PrettySpan(old.span()),
                                        ))
                                        .emit();
                                }

                                // Clone, one for `succs` and one for `preds`.
                                let (src_a, src_b) = (src_port.clone(), src_port);
                                let (dst_a, dst_b) = (dst_port.clone(), dst_port);

                                if let Some((old_a, _)) =
                                    self.operators[out].succs.remove_entry(&src_a)
                                {
                                    emit_conflict("Output", &old_a, &src_a);
                                }
                                self.operators[out].succs.insert(src_a, (inn, dst_a));

                                if let Some((old_b, _)) =
                                    self.operators[inn].preds.remove_entry(&dst_b)
                                {
                                    emit_conflict("Input", &old_b, &dst_b);
                                }
                                self.operators[inn].preds.insert(dst_b, (out, src_b));
                            }
                        }

                        let ports = Ports {
                            inn: curr_ports.inn,
                            out: next_ports.out,
                        };
                        (ports, next_arrow)
                    });

                output.map(|(ports, _arrow)| ports).unwrap_or(Ports {
                    inn: None,
                    out: None,
                })
            }
            Pipeline::Name(ident) => self.names.get(&ident).copied().unwrap_or_else(|| {
                ident
                    .span()
                    .unwrap()
                    .error(format!("Cannot find name `{}`", ident))
                    .emit();
                Ports {
                    inn: None,
                    out: None,
                }
            }),
            Pipeline::Operator(operator) => {
                let (preds, succs) = Default::default();
                let op_info = OpInfo {
                    operator,
                    preds,
                    succs,
                    subgraph_id: None,
                };
                let port = self.operators.insert(op_info);
                Ports {
                    inn: Some(port),
                    out: Some(port),
                }
            }
        }
    }

    /// Validates that operators have valid number of inputs and outputs.
    /// (Emits error messages on span).
    /// TODO(mingwei): Clean this up, make it do more than just arity.
    pub fn validate_operators(&self) {
        use std::ops::{Bound, RangeBounds};
        trait RangeTrait<T>
        where
            T: ?Sized,
        {
            fn start_bound(&self) -> Bound<&T>;
            fn end_bound(&self) -> Bound<&T>;
            fn contains(&self, item: &T) -> bool
            where
                T: PartialOrd<T>;
        }
        impl<R, T> RangeTrait<T> for R
        where
            R: RangeBounds<T>,
        {
            fn start_bound(&self) -> Bound<&T> {
                self.start_bound()
            }

            fn end_bound(&self) -> Bound<&T> {
                self.end_bound()
            }

            fn contains(&self, item: &T) -> bool
            where
                T: PartialOrd<T>,
            {
                self.contains(item)
            }
        }

        for opinfo in self.operators.values() {
            let op_name = &*opinfo.operator.path.to_token_stream().to_string();
            let (inn_allowed, out_allowed): (&dyn RangeTrait<usize>, &dyn RangeTrait<usize>) =
                match op_name {
                    "merge" => (&(2..), &(1..=1)),
                    "join" => (&(2..=2), &(1..=1)),
                    "tee" => (&(1..=1), &(2..)),
                    "map" | "dedup" => (&(1..=1), &(1..=1)),
                    "input" | "seed" => (&(0..=0), &(1..=1)),
                    "for_each" => (&(1..=1), &(0..=0)),
                    unknown => {
                        opinfo
                            .operator
                            .path
                            .span()
                            .unwrap()
                            .error(format!("Unknown operator `{}`", unknown))
                            .emit();
                        (&(..), &(..))
                    }
                };

            if !inn_allowed.contains(&opinfo.preds.len()) {
                opinfo
                    .operator
                    .span()
                    .unwrap()
                    .error(format!(
                        "`{}` has invalid number of inputs: {}. Allowed is between {:?} and {:?}.",
                        op_name,
                        &opinfo.preds.len(),
                        inn_allowed.start_bound(),
                        inn_allowed.end_bound()
                    ))
                    .emit();
            }
            if !out_allowed.contains(&opinfo.succs.len()) {
                opinfo
                    .operator
                    .span()
                    .unwrap()
                    .error(format!(
                        "`{}` has invalid number of outputs: {}. Allowed is between {:?} and {:?}.",
                        op_name,
                        &opinfo.succs.len(),
                        out_allowed.start_bound(),
                        out_allowed.end_bound()
                    ))
                    .emit();
            }
        }
    }

    pub fn identify_subgraphs(&mut self) {
        // Pull (green)
        // Push (blue)
        // Handoff (red) -- not a color for operators, inserted between subgraphs.
        // Computation (yellow)
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        enum Color {
            Pull,
            Push,
            Comp,
        }

        fn op_color(opinfo: &OpInfo) -> Option<Color> {
            match (1 < opinfo.preds.len(), 1 < opinfo.succs.len()) {
                (true, true) => Some(Color::Comp),
                (true, false) => Some(Color::Pull),
                (false, true) => Some(Color::Push),
                (false, false) => match (opinfo.preds.is_empty(), opinfo.succs.is_empty()) {
                    (true, false) => Some(Color::Pull),
                    (false, true) => Some(Color::Push),
                    _same => None,
                },
            }
        }

        fn assign_color_nexts(
            colors: &mut SecondaryMap<OperatorId, Color>,
            operators: &SlotMap<OperatorId, OpInfo>,
            id: OperatorId,
            color: Color,
            preds: bool,
        ) {
            let opinfo = operators.get(id).unwrap();
            if op_color(opinfo).map(|c| c == color).unwrap_or(true) {
                colors.insert(id, color);

                let nexts = if preds { &opinfo.preds } else { &opinfo.succs };
                for &(succ_id, _) in nexts.values() {
                    assign_color_nexts(colors, operators, succ_id, color, preds);
                }
            }
        }

        let mut colors = SecondaryMap::with_capacity(self.operators.len());
        for id in self.operators.keys() {
            if colors.contains_key(id) {
                continue;
            }

            let opinfo = self.operators.get(id).unwrap();
            if let Some(color) = op_color(opinfo) {
                colors.insert(id, color);

                match color {
                    Color::Comp => {
                        assign_color_nexts(&mut colors, &self.operators, id, Color::Pull, true);
                        assign_color_nexts(&mut colors, &self.operators, id, Color::Push, false);
                    }
                    pull_or_push => {
                        assign_color_nexts(&mut colors, &self.operators, id, pull_or_push, true);
                        assign_color_nexts(&mut colors, &self.operators, id, pull_or_push, false);
                    }
                }
            }
        }
    }

    pub fn mermaid_string(&self) -> String {
        let mut string = String::new();
        self.write_mermaid(&mut string).unwrap();
        string
    }

    pub fn write_mermaid(&self, write: &mut impl std::fmt::Write) -> std::fmt::Result {
        writeln!(write, "flowchart TB")?;
        for (key, opinfo) in self.operators.iter() {
            writeln!(
                write,
                r#"    {}["{}"]"#,
                key.data().as_ffi(),
                opinfo
                    .operator
                    .to_token_stream()
                    .to_string()
                    .replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
                    .replace('"', "&quot;")
            )?;
        }
        writeln!(write)?;
        for (src_key, op) in self.operators.iter() {
            for (_src_port, (dst_key, _dst_port)) in op.succs.iter() {
                writeln!(
                    write,
                    "    {}-->{}",
                    src_key.data().as_ffi(),
                    dst_key.data().as_ffi()
                )?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
struct Ports {
    inn: Option<OperatorId>,
    out: Option<OperatorId>,
}

struct OpInfo {
    operator: Operator,
    preds: HashMap<LitInt, (OperatorId, LitInt)>,
    succs: HashMap<LitInt, (OperatorId, LitInt)>,

    /// Which subgraph this operator belongs to (if determined).
    subgraph_id: Option<SubgraphId>,
}
impl std::fmt::Debug for OpInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpInfo")
            .field("operator (span)", &self.operator.span())
            .field("preds", &self.preds)
            .field("succs", &self.succs)
            .finish()
    }
}

/// Helper struct which displays the span as `path:row:col` for human reading/IDE linking.
/// Example: `hydroflow\tests\surface_syntax.rs:42:18`.
struct PrettySpan(Span);
impl std::fmt::Display for PrettySpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let span = self.0.unwrap();
        write!(
            f,
            "{}:{}:{}",
            span.source_file().path().display(),
            span.start().line,
            span.start().column
        )
    }
}
