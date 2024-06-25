#![allow(clippy::needless_lifetimes)]

//[flowgraph]//
//[flowgraph_sig]//
use hydroflow_plus::*;
use stageleft::*;

pub fn first_ten<'a, D: LocalDeploy<'a>>(
    flow: &FlowBuilder<'a, D>,
    process_spec: &impl ProcessSpec<'a, D>,
) {
    //[/flowgraph_sig]//
    //[process]//
    let process = flow.process(process_spec);
    //[/process]//

    //[numbers]//
    let numbers = flow.source_iter(&process, q!(0..10)); // : Stream<_, i32, _, _>
    //[/numbers]//
    //[foreach]//
    numbers.for_each(q!(|n| println!("{}", n)));
    //[/foreach]//
}
//[/flowgraph]//

//[runtime]//
#[stageleft::entry]
pub fn first_ten_runtime<'a>(
    flow: FlowBuilder<'a, SingleProcessGraph>,
) -> impl Quoted<'a, Hydroflow<'a>> {
    first_ten(&flow, &()); // &() for a single process graph.
    flow.extract().optimize_default() // : impl Quoted<'a, Hydroflow<'a>>
}
//[/runtime]//
