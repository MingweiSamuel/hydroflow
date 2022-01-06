use super::handoff_pull::HandoffPull;
use super::{Pull, PullBase, Push, PushBase};

use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::scheduled::ctx::{InputPort, OutputPort};
use crate::scheduled::graph::Hydroflow;
use crate::scheduled::handoff::{Handoff, HandoffList};
use crate::{tl, tt};

#[derive(Default)]
pub struct HydroflowBuilder {
    hydroflow: Hydroflow,
    handoff_connectors: Vec<Box<dyn FnOnce(&mut Hydroflow)>>, // TODO(mingwei): this is a janky/unprincipled way to do this.
                                                              // inputs: HashMap<&'static str, Box<dyn Any>>,
}
impl HydroflowBuilder {
    pub fn make_handoff<H>(&mut self) -> (BuilderHandoffPush<H>, BuilderHandoffPull<H>)
    where
        H: Handoff,
    {
        let push = BuilderHandoffPush {
            port: Default::default(),
            _phantom: PhantomData,
        };
        let pull = BuilderHandoffPull {
            port: Default::default(),
            _phantom: PhantomData,
        };
        let push_port = Rc::clone(&push.port);
        let pull_port = Rc::clone(&pull.port);
        self.handoff_connectors.push(Box::new(move |hydroflow| {
            if let (Some(output_port), Some(input_port)) = (push_port.take(), pull_port.take()) {
                hydroflow.add_edge(output_port, input_port);
            } else {
                panic!("Handoff was never connected!!"); // TODO(mingwei): more informative error messages.
            }
        }));
        (push, pull)
    }

    pub fn build(mut self) -> Hydroflow {
        for handoff_connector in self.handoff_connectors {
            // TODO(mingwei): be more principled with this.
            (handoff_connector)(&mut self.hydroflow);
        }
        self.hydroflow
    }
}

pub struct BuilderHandoffPull<H>
where
    H: Handoff,
{
    port: Rc<Cell<Option<InputPort<H>>>>,
    _phantom: PhantomData<*const H>,
}
impl<H> PullBase for BuilderHandoffPull<H>
where
    H: Handoff,
{
    type Item = H::Inner;
    type Build<'i> = std::array::IntoIter<Self::Item, 1>;
}
impl<H> Pull for BuilderHandoffPull<H>
where
    H: Handoff,
{
    type InputHandoffs = tt!(H);

    fn init(&mut self, input_ports: <Self::InputHandoffs as HandoffList>::InputPort) {
        let tl!(input_port) = input_ports;
        let old_val = self.port.replace(Some(input_port));
        assert!(old_val.is_none());
    }

    /// Builds the iterator for a single run of the subgraph.
    fn build(
        &mut self,
        input: <Self::InputHandoffs as HandoffList>::RecvCtx<'_>,
    ) -> Self::Build<'_> {
        let tl!(handoff) = input;
        [handoff.take_inner()].into_iter()
    }
}

pub struct BuilderHandoffPush<H>
where
    H: Handoff,
{
    port: Rc<Cell<Option<OutputPort<H>>>>,
    _phantom: PhantomData<*const H>,
}
