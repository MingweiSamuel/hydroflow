use super::pivot::Pivot;
use super::{IdentityPushBuild, Pull, PullBase, Push, PushBase};

use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::mpsc::SyncSender;

use crate::compiled::push_handoff::PushHandoff;
use crate::scheduled::ctx::{InputPort, OutputPort};
use crate::scheduled::graph::Hydroflow;
use crate::scheduled::graph_ext::GraphExt;
use crate::scheduled::handoff::{CanReceive, Handoff, HandoffList};
use crate::scheduled::input::Input;
use crate::{tl, tt};

#[derive(Default)]
pub struct HydroflowBuilder {
    hydroflow: Hydroflow,
    // TODO(mingwei): this is a janky/unprincipled way to do this.
    // inputs: HashMap<&'static str, Box<dyn Any>>,
    handoff_connectors: Vec<Box<dyn FnOnce(&mut Hydroflow)>>,
}
impl HydroflowBuilder {
    pub fn make_handoff<H, T>(&mut self) -> (BuilderHandoffPush<H, T>, BuilderHandoffPull<H>)
    where
        H: Handoff + CanReceive<T>,
    {
        let push = BuilderHandoffPush {
            port: Default::default(),
            _phantom: PhantomData,
        };
        let pull = BuilderHandoffPull {
            port: Default::default(),
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

    pub fn add_subgraph<I, O>(&mut self, mut pivot: Pivot<I, O>)
    where
        I: 'static + Pull,
        O: 'static + Push<Item = I::Item>,
    {
        // TODO(mingwei): extremely jank
        let pivot_transfer: Rc<RefCell<Option<Pivot<I, O>>>> = Default::default();
        let pivot_send = Rc::clone(&pivot_transfer);

        let (input_ports, output_ports) = self
            .hydroflow
            .add_subgraph::<_, I::InputHandoffs, O::OutputHandoffs>(
                move |ctx, recv_ctx, send_ctx| {
                    let mut pivot_borrow = pivot_transfer.borrow_mut();
                    pivot_borrow
                        .as_mut()
                        .expect("Failed to set pivot_send")
                        .run(ctx, recv_ctx, send_ctx);
                },
            );

        pivot.init(input_ports, output_ports);
        *pivot_send.borrow_mut() = Some(pivot);
    }

    pub fn add_channel_input<T, W>(&mut self) -> (Input<T, SyncSender<T>>, BuilderHandoffPull<W>)
    where
        T: 'static,
        W: 'static + Handoff + CanReceive<T>,
    {
        let (input, output_port) = self.hydroflow.add_channel_input();

        let pull = BuilderHandoffPull {
            port: Default::default(),
        };
        let pull_port = Rc::clone(&pull.port);

        self.handoff_connectors.push(Box::new(move |hydroflow| {
            if let Some(input_port) = pull_port.take() {
                hydroflow.add_edge(output_port, input_port);
            } else {
                panic!("Channel input was never connected!!"); // TODO(mingwei): more informative error messages.
            }
        }));

        (input, pull)
    }

    pub fn build(mut self) -> Hydroflow {
        for handoff_connector in self.handoff_connectors {
            // TODO(mingwei): be more principled with this.
            (handoff_connector)(&mut self.hydroflow);
        }
        self.hydroflow
    }

    pub fn start_tee<T>(&self) -> IdentityPushBuild<T> {
        IdentityPushBuild::new()
    }
}

pub struct BuilderHandoffPull<H>
where
    H: Handoff,
{
    port: Rc<Cell<Option<InputPort<H>>>>,
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

pub struct BuilderHandoffPush<H, T>
where
    H: Handoff + CanReceive<T>,
{
    port: Rc<Cell<Option<OutputPort<H>>>>,
    _phantom: PhantomData<fn(T)>,
}
impl<H, T> PushBase for BuilderHandoffPush<H, T>
where
    H: Handoff + CanReceive<T>,
{
    type Item = T;
    type Build<'a, 'i> = PushHandoff<'i, H, T>;
}
impl<H, T> Push for BuilderHandoffPush<H, T>
where
    H: Handoff + CanReceive<T>,
{
    type OutputHandoffs = tt!(H);

    fn init(&mut self, output_ports: <Self::OutputHandoffs as HandoffList>::OutputPort) {
        let tl!(output_port) = output_ports;
        let old_val = self.port.replace(Some(output_port));
        assert!(old_val.is_none());
    }

    fn build<'a, 'i>(
        &'a mut self,
        input: <Self::OutputHandoffs as HandoffList>::SendCtx<'i>,
    ) -> Self::Build<'a, 'i> {
        let tl!(handoff) = input;
        PushHandoff::new(handoff)
    }
}
