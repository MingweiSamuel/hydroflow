use super::context::Context;
use super::graph::FlowGraph;

/**
 * Represents a compiled subgraph. Used internally by [Dataflow] to erase the input/output [Handoff] types.
 */
pub(crate) trait Subgraph {
    // TODO: pass in some scheduling info?
    /// Run the subgraph for one quantum of time.
    /// For now that is as much work as is available.
    fn run(&mut self, context: Context<'_>);

    /// Write the subgraph to `flow_graph`. By default does nothing.
    fn write_flow_graph(&self, _flow_graph: &mut FlowGraph) {}
}
impl<F> Subgraph for F
where
    F: FnMut(Context<'_>),
{
    fn run(&mut self, context: Context<'_>) {
        (self)(context);
    }
}
