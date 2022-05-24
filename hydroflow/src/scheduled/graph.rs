use std::any::Any;
use std::borrow::Cow;
use std::cell::Cell;
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::num::NonZeroUsize;

use ref_cast::RefCast;
use tokio::sync::mpsc::{self, UnboundedReceiver};

use super::context::Context;
use super::flow_graph::FlowGraph;
use super::handoff::handoff_list::PortList;
use super::handoff::{Handoff, HandoffMeta};
use super::port::{RecvCtx, RecvPort, SendCtx, SendPort, RECV, SEND};
use super::reactor::Reactor;
use super::state::StateHandle;
use super::subgraph::Subgraph;
use super::{HandoffId, SubgraphId};

/// A Hydroflow graph. Owns, schedules, and runs the compiled subgraphs.
pub struct Hydroflow {
    pub(super) subgraphs: Vec<SubgraphData>,
    pub(super) context: Context,
    scheduler: Scheduler,
}
impl Default for Hydroflow {
    fn default() -> Self {
        let (subgraphs, handoffs, states) = Default::default();
        let stratum_queues = vec![Default::default()]; // Always initialize stratum #0.
        let (event_queue_send, event_queue_recv) = mpsc::unbounded_channel();
        let context = Context {
            handoffs,
            states,

            event_queue_send,

            current_epoch: 0,
            current_stratum: 0,

            subgraph_id: SubgraphId(0),
        };
        let scheduler = Scheduler {
            stratum_queues,
            event_queue_recv,

            current_epoch: 0,
            current_stratum: 0,
        };
        Self {
            subgraphs,
            context,
            scheduler,
        }
    }
}
impl Hydroflow {
    /// Create a new empty Hydroflow graph.
    pub fn new() -> Self {
        Default::default()
    }

    /// Returns a reactor for externally scheduling subgraphs, possibly from another thread.
    pub fn reactor(&self) -> Reactor {
        Reactor::new(self.context.event_queue_send.clone())
    }

    // Gets the current epoch (local time) count.
    pub fn current_epoch(&self) -> usize {
        self.context.current_epoch
    }

    // Gets the current stratum nubmer.
    pub fn current_stratum(&self) -> usize {
        self.context.current_stratum
    }

    /// Runs the dataflow until the next epoch begins.
    pub fn run_epoch(&mut self) {
        let epoch = self.current_epoch();
        while self.scheduler.next_stratum(&*self.subgraphs) && epoch == self.current_epoch() {
            self.run_stratum();
        }
    }

    /// Runs the dataflow until no more work is immediately available.
    /// If the dataflow contains loops this method may run forever.
    pub fn run_available(&mut self) {
        while self.scheduler.next_stratum(&*self.subgraphs) {
            self.run_stratum();
        }
    }

    /// Runs the current stratum of the dataflow until no more work is immediately available.
    pub fn run_stratum(&mut self) {
        // Add any external jobs to ready queue.
        self.scheduler.try_recv_events(&*self.subgraphs);

        while let Some(sg_id) = self.scheduler.pop_current_stratum() {
            {
                let sg_data = &mut self.subgraphs[sg_id.0];
                // This must be true for the subgraph to be enqueued.
                assert!(sg_data.is_scheduled.take());

                self.context.subgraph_id = sg_id;
                sg_data.subgraph.run(&mut self.context);
            }

            for &handoff_id in self.subgraphs[sg_id.0].succs.iter() {
                let handoff = &self.context.handoffs[handoff_id.0];
                if !handoff.handoff.is_bottom() {
                    for &succ_id in handoff.succs.iter() {
                        self.scheduler.schedule(succ_id, &self.subgraphs);
                    }
                }
            }

            self.scheduler.try_recv_events(&*self.subgraphs);
        }
    }

    /// Runs the dataflow graph forever.
    ///
    /// TODO(mingwei): Currently blockes forever, no notion of "completion."
    pub fn run(&mut self) -> Option<!> {
        loop {
            self.run_available();
            self.scheduler.recv_events(&*self.subgraphs)?;
        }
    }

    /// Runs the dataflow graph forever.
    ///
    /// TODO(mingwei): Currently blockes forever, no notion of "completion."
    pub async fn run_async(&mut self) -> Option<!> {
        loop {
            self.run_available();
            self.scheduler.recv_events_async(&*self.subgraphs).await?;
            tokio::task::yield_now().await;
        }
    }

    pub fn add_subgraph<Name, R, W, F>(
        &mut self,
        name: Name,
        recv_ports: R,
        send_ports: W,
        subgraph: F,
    ) -> SubgraphId
    where
        Name: Into<Cow<'static, str>>,
        R: 'static + PortList<RECV>,
        W: 'static + PortList<SEND>,
        F: 'static + for<'ctx> FnMut(&'ctx Context, R::Ctx<'ctx>, W::Ctx<'ctx>),
    {
        self.add_subgraph_stratified(name, 0, recv_ports, send_ports, subgraph)
    }

    /// Adds a new compiled subgraph with the specified inputs and outputs.
    ///
    /// TODO(mingwei): add example in doc.
    pub fn add_subgraph_stratified<Name, R, W, F>(
        &mut self,
        name: Name,
        stratum: usize,
        recv_ports: R,
        send_ports: W,
        mut subgraph: F,
    ) -> SubgraphId
    where
        Name: Into<Cow<'static, str>>,
        R: 'static + PortList<RECV>,
        W: 'static + PortList<SEND>,
        F: 'static + for<'ctx> FnMut(&'ctx Context, R::Ctx<'ctx>, W::Ctx<'ctx>),
    {
        let sg_id = SubgraphId(self.subgraphs.len());

        let (mut subgraph_preds, mut subgraph_succs) = Default::default();
        recv_ports.set_graph_meta(
            &mut *self.context.handoffs,
            None,
            Some(sg_id),
            &mut subgraph_preds,
        );
        send_ports.set_graph_meta(
            &mut *self.context.handoffs,
            Some(sg_id),
            None,
            &mut subgraph_succs,
        );

        let subgraph = move |context: &mut Context| {
            let recv = recv_ports.make_ctx(&*context.handoffs);
            let send = send_ports.make_ctx(&*context.handoffs);
            (subgraph)(context, recv, send);
        };
        self.subgraphs.push(SubgraphData::new(
            name.into(),
            stratum,
            subgraph,
            subgraph_preds,
            subgraph_succs,
            FlowGraph::default(),
            true,
        ));
        self.scheduler.schedule(sg_id, &*self.subgraphs);

        sg_id
    }

    /// Adds a new compiled subgraph with a variable number of inputs and outputs of the same respective handoff types.
    pub fn add_subgraph_n_m<Name, R, W, F>(
        &mut self,
        name: Name,
        recv_ports: Vec<RecvPort<R>>,
        send_ports: Vec<SendPort<W>>,
        subgraph: F,
    ) -> SubgraphId
    where
        Name: Into<Cow<'static, str>>,
        R: 'static + Handoff,
        W: 'static + Handoff,
        F: 'static
            + for<'ctx> FnMut(&'ctx Context, &'ctx [&'ctx RecvCtx<R>], &'ctx [&'ctx SendCtx<W>]),
    {
        self.add_subgraph_stratified_n_m(name, 0, recv_ports, send_ports, subgraph)
    }

    /// Adds a new compiled subgraph with a variable number of inputs and outputs of the same respective handoff types.
    pub fn add_subgraph_stratified_n_m<Name, R, W, F>(
        &mut self,
        name: Name,
        stratum: usize,
        recv_ports: Vec<RecvPort<R>>,
        send_ports: Vec<SendPort<W>>,
        mut subgraph: F,
    ) -> SubgraphId
    where
        Name: Into<Cow<'static, str>>,
        R: 'static + Handoff,
        W: 'static + Handoff,
        F: 'static
            + for<'ctx> FnMut(&'ctx Context, &'ctx [&'ctx RecvCtx<R>], &'ctx [&'ctx SendCtx<W>]),
    {
        let sg_id = SubgraphId(self.subgraphs.len());

        let subgraph_preds = recv_ports.iter().map(|port| port.handoff_id).collect();
        let subgraph_succs = send_ports.iter().map(|port| port.handoff_id).collect();

        for recv_port in recv_ports.iter() {
            self.context.handoffs[recv_port.handoff_id.0]
                .succs
                .push(sg_id);
        }
        for send_port in send_ports.iter() {
            self.context.handoffs[send_port.handoff_id.0]
                .preds
                .push(sg_id);
        }

        let subgraph = move |context: &mut Context| {
            let recvs: Vec<&RecvCtx<R>> = recv_ports
                .iter()
                .map(|hid| hid.handoff_id)
                .map(|hid| context.handoffs.get(hid.0).unwrap())
                .map(|h_data| {
                    h_data
                        .handoff
                        .any_ref()
                        .downcast_ref()
                        .expect("Attempted to cast handoff to wrong type.")
                })
                .map(RefCast::ref_cast)
                .collect();

            let sends: Vec<&SendCtx<W>> = send_ports
                .iter()
                .map(|hid| hid.handoff_id)
                .map(|hid| context.handoffs.get(hid.0).unwrap())
                .map(|h_data| {
                    h_data
                        .handoff
                        .any_ref()
                        .downcast_ref()
                        .expect("Attempted to cast handoff to wrong type.")
                })
                .map(RefCast::ref_cast)
                .collect();

            (subgraph)(context, &recvs, &sends)
        };
        self.subgraphs.push(SubgraphData::new(
            name.into(),
            stratum,
            subgraph,
            subgraph_preds,
            subgraph_succs,
            FlowGraph::default(),
            true,
        ));
        self.scheduler.schedule(sg_id, &*self.subgraphs);

        sg_id
    }

    /// Creates a handoff edge and returns the corresponding send and receive ports.
    pub fn make_edge<Name, H>(&mut self, name: Name) -> (SendPort<H>, RecvPort<H>)
    where
        Name: Into<Cow<'static, str>>,
        H: 'static + Handoff,
    {
        let handoff_id = HandoffId(self.context.handoffs.len());

        // Create and insert handoff.
        let handoff = H::default();
        self.context
            .handoffs
            .push(HandoffData::new(name.into(), handoff));

        // Make ports.
        let input_port = SendPort {
            handoff_id,
            _marker: PhantomData,
        };
        let output_port = RecvPort {
            handoff_id,
            _marker: PhantomData,
        };
        (input_port, output_port)
    }

    pub fn add_state<T>(&mut self, state: T) -> StateHandle<T>
    where
        T: Any,
    {
        self.context.add_state(state)
    }

    /// Gets a exclusive (mut) ref to the internal context, setting the subgraph ID.
    pub fn context_mut(&mut self, sg_id: SubgraphId) -> &mut Context {
        self.context.subgraph_id = sg_id;
        &mut self.context
    }

    pub(crate) fn next_subgraph_id(&self) -> SubgraphId {
        SubgraphId(self.subgraphs.len())
    }

    pub fn add_dependencies(&mut self, sg_id: SubgraphId, deps: FlowGraph) {
        self.subgraphs[sg_id.0].dependencies.append(deps);
    }
}

/// Scheduling component of a [`Hydroflow`] graph.
struct Scheduler {
    /// Index is stratum, value is FIFO queue for that stratum.
    stratum_queues: Vec<VecDeque<SubgraphId>>,
    event_queue_recv: UnboundedReceiver<SubgraphId>,

    current_epoch: usize,
    current_stratum: usize,
}
impl Scheduler {
    /// Schedules the given `sg_id`. Returns `true` if the subgraph is
    /// scheduled. Returns `false` if subgraph was already scheduled and no
    /// changes were made.
    pub fn schedule(&mut self, sg_id: SubgraphId, subgraphs: &[SubgraphData]) -> bool {
        let sg_data = &subgraphs[sg_id.0];
        if sg_data.is_scheduled.replace(true) {
            // Already scheduled, skip.
            false
        } else {
            self.init_stratum(sg_data.stratum);
            self.stratum_queues[sg_data.stratum].push_back(sg_id);
            true
        }
    }

    /// Makes sure stratum STRATUM is initialized.
    fn init_stratum(&mut self, stratum: usize) {
        if self.stratum_queues.len() <= stratum {
            self.stratum_queues
                .resize_with(stratum + 1, Default::default);
        }
    }

    /// Enqueues subgraphs triggered by external events without blocking.
    ///
    /// Returns the number of subgraphs enqueued.
    pub fn try_recv_events(&mut self, subgraphs: &[SubgraphData]) -> usize {
        let mut enqueued_count = 0;
        while let Ok(sg_id) = self.event_queue_recv.try_recv() {
            if self.schedule(sg_id, subgraphs) {
                enqueued_count += 1;
            }
        }
        enqueued_count
    }

    /// Enqueues subgraphs triggered by external events, blocking until at
    /// least one subgraph is scheduled.
    pub fn recv_events(&mut self, subgraphs: &[SubgraphData]) -> Option<NonZeroUsize> {
        loop {
            let sg_id = self.event_queue_recv.blocking_recv()?;
            if self.schedule(sg_id, subgraphs) {
                // Enqueue any other immediate events.
                return Some(NonZeroUsize::new(self.try_recv_events(subgraphs) + 1).unwrap());
            }
        }
    }

    /// Enqueues subgraphs triggered by external events asynchronously, waiting
    /// until at least one subgraph is scheduled.
    pub async fn recv_events_async(&mut self, subgraphs: &[SubgraphData]) -> Option<NonZeroUsize> {
        loop {
            let sg_id = self.event_queue_recv.recv().await?;
            if self.schedule(sg_id, subgraphs) {
                // Enqueue any other immediate events.
                return Some(NonZeroUsize::new(self.try_recv_events(subgraphs) + 1).unwrap());
            }
        }
    }

    /// Go to the next stratum which has work available, possibly the current stratum.
    /// Return true if more work is available, otherwise false if no work is immediately available on any strata.
    pub fn next_stratum(&mut self, subgraphs: &[SubgraphData]) -> bool {
        self.try_recv_events(subgraphs);

        let old_stratum = self.current_stratum;
        loop {
            // If current stratum has work, return true.
            if !self.stratum_queues[self.current_stratum].is_empty() {
                return true;
            }
            // Increment stratum counter.
            self.current_stratum += 1;
            if self.current_stratum >= self.stratum_queues.len() {
                self.current_stratum = 0;
                self.current_epoch += 1;
            }
            // After incrementing, exit if we made a full loop around the strata.
            if old_stratum == self.current_stratum {
                // Note: if current stratum had work, the very first loop iteration would've
                // returned true. Therefore we can return false without checking.
                return false;
            }
        }
    }

    /// Pops a `SubgraphId` from the current stratum queue, or `None` if none
    /// available.
    pub fn pop_current_stratum(&mut self) -> Option<SubgraphId> {
        self.stratum_queues[self.current_stratum].pop_front()
    }
}

/// A handoff and its input and output [SubgraphId]s.
///
/// Internal use: used to track the hydroflow graph structure.
///
/// TODO(mingwei): restructure `PortList` so this can be crate-private.
pub struct HandoffData {
    /// A friendly name for diagnostics.
    #[allow(dead_code)] // TODO(mingwei): remove attr once used.
    pub(super) name: Cow<'static, str>,
    /// Crate-visible to crate for `handoff_list` internals.
    pub(super) handoff: Box<dyn HandoffMeta>,
    pub(super) preds: Vec<SubgraphId>,
    pub(super) succs: Vec<SubgraphId>,
}
impl std::fmt::Debug for HandoffData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("HandoffData")
            .field("preds", &self.preds)
            .field("succs", &self.succs)
            .finish_non_exhaustive()
    }
}
impl HandoffData {
    pub fn new(name: Cow<'static, str>, handoff: impl 'static + HandoffMeta) -> Self {
        let (preds, succs) = Default::default();
        Self {
            name,
            handoff: Box::new(handoff),
            preds,
            succs,
        }
    }
}

/// A subgraph along with its predecessor and successor [SubgraphId]s.
///
/// Used internally by the [Hydroflow] struct to represent the dataflow graph
/// structure and scheduled state.
pub(super) struct SubgraphData {
    /// A friendly name for diagnostics.
    #[allow(dead_code)] // TODO(mingwei): remove attr once used.
    pub(super) name: Cow<'static, str>,
    /// This subgraph's stratum number.
    pub(super) stratum: usize,
    /// The actual execution code of the subgraph.
    subgraph: Box<dyn Subgraph>,
    #[allow(dead_code)]
    preds: Vec<HandoffId>,
    succs: Vec<HandoffId>,

    pub(super) dependencies: FlowGraph,

    /// If this subgraph is scheduled in [`Hydroflow::stratum_queues`].
    /// [`Cell`] allows modifying this field when iterating `Self::preds` or
    /// `Self::succs`, as all `SubgraphData` are owned by the same vec
    /// `Hydroflow::subgraphs`.
    is_scheduled: Cell<bool>,
}
impl SubgraphData {
    pub fn new(
        name: Cow<'static, str>,
        stratum: usize,
        subgraph: impl 'static + Subgraph,
        preds: Vec<HandoffId>,
        succs: Vec<HandoffId>,
        dependencies: FlowGraph,
        is_scheduled: bool,
    ) -> Self {
        Self {
            name,
            stratum,
            subgraph: Box::new(subgraph),
            preds,
            succs,
            dependencies,
            is_scheduled: Cell::new(is_scheduled),
        }
    }
}

/// Internal struct containing a pointer to [`Hydroflow`]-owned state.
pub(crate) struct StateData {
    pub state: Box<dyn Any>,
}
