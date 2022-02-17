#![cfg(feature = "graphviz")]

use super::graph::Hydroflow;

use graphviz_rust::dot_structures::{
    Attribute, Edge, EdgeTy, Graph, Id, Node, NodeId, Stmt, Subgraph,
};
use graphviz_rust::printer::{DotPrinter, PrinterContext};

pub trait Graphviz {
    fn graphviz(&self, output: &mut Vec<Stmt>);

    fn dump_graphviz(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        use std::io::Write;

        let mut stmts = Vec::new();
        self.graphviz(&mut stmts);

        let graph = Graph::DiGraph {
            id: Id::Plain("hydroflow".to_owned()),
            strict: false,
            stmts,
        };

        let mut ctx = PrinterContext::default();
        ctx.with_indent_step(2);
        let graph = graph.print(&mut ctx);

        let mut file = std::io::BufWriter::new(std::fs::File::create(path)?);
        write!(file, "{}", graph)?;
        Ok(())
    }
}

impl Graphviz for Hydroflow {
    fn graphviz(&self, output: &mut Vec<Stmt>) {
        let label_id = Id::Plain("label".to_owned());
        let shape_id = Id::Plain("shape".to_owned());
        let style_id = Id::Plain("style".to_owned());

        output.extend(
            self.subgraphs
                .iter()
                .enumerate()
                .map(|(sg_id, sg_data)| {
                    let preds = sg_data.preds.iter().map(|&hoff_id| (hoff_id, "recv"));
                    let succs = sg_data.succs.iter().map(|&hoff_id| (hoff_id, "send"));
                    Subgraph {
                        id: Id::Plain(format!("cluster_sg_{}", sg_id)),
                        stmts: preds
                            .chain(succs)
                            .map(|(hoff_id, send_recv)| Node {
                                id: NodeId(
                                    Id::Plain(format!(
                                        "sg_{}_hoff_{}_{}",
                                        sg_id, hoff_id, send_recv
                                    )),
                                    None,
                                ),
                                attributes: vec![Attribute(
                                    label_id.clone(),
                                    Id::Plain(format!(
                                        "{:?}",
                                        format!("H[{}] {}", hoff_id, send_recv)
                                    )),
                                )],
                            })
                            .map(From::from)
                            .chain(
                                [
                                    Attribute(
                                        label_id.clone(),
                                        Id::Plain(format!(
                                            "{:?}",
                                            format!("SG[{}]: {}", sg_id, sg_data.name)
                                        )),
                                    ),
                                    Attribute(style_id.clone(), Id::Plain("rounded".to_owned())),
                                ]
                                .into_iter()
                                .map(From::from),
                            )
                            .collect(),
                    }
                })
                .map(From::from),
        );
        output.extend(
            self.handoffs
                .iter()
                .enumerate()
                .flat_map(|(hoff_id, hoff_data)| {
                    let hoff_node_id_a = NodeId(Id::Plain(format!("hoff_{}", hoff_id)), None);
                    let hoff_node_id_b = hoff_node_id_a.clone();
                    let hoff_node_id_c = hoff_node_id_a.clone();
                    let preds = hoff_data.preds.iter().map(move |&sg_id| {
                        EdgeTy::Pair(
                            NodeId(
                                Id::Plain(format!("sg_{}_hoff_{}_send", sg_id, hoff_id)),
                                None,
                            )
                            .into(),
                            hoff_node_id_a.clone().into(),
                        )
                    });
                    let succs = hoff_data.succs.iter().map(move |&sg_id| {
                        EdgeTy::Pair(
                            hoff_node_id_b.clone().into(),
                            NodeId(
                                Id::Plain(format!("sg_{}_hoff_{}_recv", sg_id, hoff_id)),
                                None,
                            )
                            .into(),
                        )
                    });
                    let hoff_node = Node {
                        id: hoff_node_id_c,
                        attributes: vec![
                            Attribute(
                                label_id.clone(),
                                Id::Plain(format!(
                                    "{:?}",
                                    format!("H[{}]: {}", hoff_id, hoff_data.name)
                                )),
                            ),
                            Attribute(shape_id.clone(), Id::Plain("box".to_owned())),
                        ],
                    };
                    preds
                        .chain(succs)
                        .map(|ty| Edge {
                            ty,
                            attributes: Vec::new(),
                        })
                        .map(From::from)
                        .chain([hoff_node.into()])
                }),
        );
    }
}
