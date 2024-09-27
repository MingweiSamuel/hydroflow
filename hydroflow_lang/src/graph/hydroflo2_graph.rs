use std::collections::BTreeMap;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use slotmap::{Key, KeyData};
use syn::Ident;

use super::{graph_algorithms, GraphNodeId, HydroflowGraph};
use crate::diagnostic::{Diagnostic, Level};
use crate::graph::ops::{null_write_iterator_fn, OperatorWriteOutput, WriteContextArgs};
use crate::graph::GraphSubgraphId;

impl HydroflowGraph {
    pub fn as_hydroflo2_code(
        &mut self,
        root: TokenStream,
        diagnostics: &mut Vec<Diagnostic>,
    ) -> Result<TokenStream, ()> {
        let hf = Ident::new("hf", Span::call_site());
        let context = Ident::new("context", Span::call_site());

        // Topological sort (of strongly connected components). SCC must be contained within a loop context.
        let (scc_reps, topo_sort) = graph_algorithms::topo_sort_scc(
            || self.node_ids(),
            |v| self.node_predecessor_nodes(v),
            |u| self.node_successor_nodes(u),
        );

        let mut scc_groups = BTreeMap::<GraphNodeId, Vec<GraphNodeId>>::new();
        for (node, scc_rep) in scc_reps {
            scc_groups.entry(scc_rep).or_default().push(node);
        }

        for scc_group in scc_groups.values() {
            let loop_id = self.node_loop(scc_group[0]);
            for &node_id in scc_group[1..].iter() {
                if loop_id != self.node_loop(node_id) {
                    // TODO(mingwei): better error message.
                    diagnostics.push(Diagnostic::spanned(
                        self.node(node_id).span(),
                        Level::Error,
                        "cycle spans different loop contexts",
                    ));
                    return Err(());
                }
            }
        }

        let mut op_prologue_code = Vec::new();
        let mut subgraph_op_iter_code = Vec::new();
        let mut subgraph_op_iter_after_code = Vec::new();

        for node_id in topo_sort {
            let node = self.node(node_id);
            let op_span = node.span();
            let op_inst = self.node_op_inst(node_id).unwrap();
            let op_name = op_inst.op_constraints.name;

            // TODO clean this up.
            // Collect input arguments (predecessors).
            let mut input_edges = self
                .graph
                .predecessor_edges(node_id)
                .map(|edge_id| (self.edge_ports(edge_id).1, edge_id))
                .collect::<Vec<_>>();
            // Ensure sorted by port index.
            input_edges.sort();

            let inputs = input_edges
                .iter()
                .map(|&(_port, edge_id)| {
                    let (pred, succ) = self.edge(edge_id);
                    Ident::new(
                        &format!("edge_{:?}_{:?}", pred.data(), succ.data()),
                        op_span,
                    )
                })
                .collect::<Vec<_>>();

            // Collect output arguments (successors).
            let mut output_edges = self
                .graph
                .successor_edges(node_id)
                .map(|edge_id| (self.edge_ports(edge_id).0, edge_id))
                .collect::<Vec<_>>();
            // Ensure sorted by port index.
            output_edges.sort();

            let outputs = output_edges
                .iter()
                .map(|&(_port, edge_id)| {
                    let (pred, succ) = self.edge(edge_id);
                    Ident::new(
                        &format!("edge_{:?}_{:?}", pred.data(), succ.data()),
                        op_span,
                    )
                })
                .collect::<Vec<_>>();

            let ident = outputs.get(0).cloned().unwrap_or_else(|| {
                format_ident!("__unreachable_op_{}", format!("{:?}", node_id.0))
            });

            let context_args = WriteContextArgs {
                root: &root,
                context: &context,
                hydroflow: &hf,
                subgraph_id: GraphSubgraphId(KeyData::from_ffi(0x00000001_00000001)), // Dummy
                node_id,
                op_span,
                ident: &ident,
                is_pull: true,
                inputs: &inputs,
                outputs: &outputs,
                singleton_output_ident: &format_ident!("__todo"),
                op_name,
                op_inst,
                arguments: &op_inst.arguments_pre,
                arguments_handles: &Default::default(), // Dummy
            };

            let write_result = (op_inst.op_constraints.write_fn)(&context_args, diagnostics);
            let OperatorWriteOutput {
                write_prologue,
                write_iterator,
                write_iterator_after,
            } = write_result.unwrap_or_else(|()| {
                assert!(
                    diagnostics.iter().any(Diagnostic::is_error),
                    "Operator `{}` returned `Err` but emitted no diagnostics, this is a Hydroflow bug.",
                    op_name,
                );
                OperatorWriteOutput { write_iterator: null_write_iterator_fn(&context_args), ..Default::default() }
            });

            op_prologue_code.push(write_prologue);
            subgraph_op_iter_code.push(write_iterator);
            subgraph_op_iter_after_code.push(write_iterator_after);
        }

        Ok(quote! {
            {
                #( #op_prologue_code )*

                let #context = ();
                #( #subgraph_op_iter_code )*

                #( #subgraph_op_iter_after_code )*
            }
        })
    }
}
