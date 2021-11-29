use ref_cast::RefCast;
use taskpool::slotmap::{DefaultKey, Key};
use taskpool::{Context, StateHandle, TaskContext, Taskpool};

// mod handoff_list;
// pub use handoff_list::HandoffList;

pub mod ctx;
use ctx::{InputPort, OutputPort, RecvCtx, SendCtx};

pub mod handoff;
use handoff::Handoff;

pub struct Hydroflow<Sid = DefaultKey, Tid = DefaultKey>
where
    Sid: 'static + Key,
    Tid: 'static + Key,
{
    taskpool: Taskpool<Sid, Tid>,
}
impl<Sid, Tid> Default for Hydroflow<Sid, Tid>
where
    Sid: Key,
    Tid: Key,
{
    fn default() -> Self {
        Self {
            taskpool: Default::default(),
        }
    }
}
impl Hydroflow {
    /**
     * Create an new empty Dataflow graph with default keys.
     */
    pub fn new() -> Self {
        Default::default()
    }
}
impl<Sid, Tid> Hydroflow<Sid, Tid>
where
    Sid: Key,
    Tid: Key,
{
    /**
     * Create an new empty Dataflow graph with specified keys.
     */
    pub fn with_key() -> Self {
        Default::default()
    }

    pub fn add_inout<F, R, W>(
        &mut self,
        mut subgraph: F,
    ) -> (Tid, InputPort<R, Sid, Tid>, OutputPort<W, Sid, Tid>)
    where
        F: 'static + FnMut(&TaskContext<'_, Sid, Tid>, &RecvCtx<R>, &SendCtx<W>),
        R: 'static + Handoff,
        W: 'static + Handoff,
    {
        let r_hid = self
            .taskpool
            .default_state::<Option<StateHandle<HandoffData<R, _>, _>>>();
        let w_hid = self
            .taskpool
            .default_state::<Option<StateHandle<HandoffData<W, _>, _>>>();

        let tid = self.taskpool.new_task(move |mut ctx: TaskContext<'_, _, _>| {
            let r_hid = ctx.get_state_ref(r_hid).expect("Handoff not connected.");
            let w_hid = ctx.get_state_ref(w_hid).expect("Handoff not connected.");

            let recv_handoff_data = ctx.get_state_ref(r_hid);
            let send_handoff_data = ctx.get_state_ref(w_hid);

            let recv_ctx = RefCast::ref_cast(&recv_handoff_data.handoff);
            let send_ctx = RefCast::ref_cast(&send_handoff_data.handoff);

            (subgraph)(&ctx, recv_ctx, send_ctx);

            if !send_handoff_data.handoff.is_bottom() {
                // Schedule the next op.
                let succ = send_handoff_data.succ;
                ctx.schedule(succ);
            }
        });

        let input_port = InputPort { subgraph: tid, handle: r_hid };
        let output_port = OutputPort { subgraph: tid, handle: w_hid };
        (tid, input_port, output_port)
    }

    pub fn add_source<F, W>(
        &mut self,
        mut subgraph: F,
    ) -> (Tid, OutputPort<W, Sid, Tid>)
    where
        F: 'static + FnMut(&TaskContext<'_, Sid, Tid>, &SendCtx<W>),
        W: 'static + Handoff,
    {
        let w_hid = self
            .taskpool
            .default_state::<Option<StateHandle<HandoffData<W, _>, _>>>();

        let tid = self.taskpool.new_task(move |mut ctx: TaskContext<'_, _, _>| {
            let w_hid = ctx.get_state_ref(w_hid).expect("Handoff not connected.");
            let send_handoff_data = ctx.get_state_ref(w_hid);
            let send_ctx = RefCast::ref_cast(&send_handoff_data.handoff);

            (subgraph)(&ctx, send_ctx);

            if !send_handoff_data.handoff.is_bottom() {
                // Schedule the next op.
                let succ = send_handoff_data.succ;
                ctx.schedule(succ);
            }
        });

        // EXTRA for sources.
        self.taskpool.schedule(tid);

        let output_port = OutputPort { subgraph: tid, handle: w_hid };
        (tid, output_port)
    }

    pub fn add_sink<F, R>(
        &mut self,
        mut subgraph: F,
    ) -> (Tid, InputPort<R, Sid, Tid>)
    where
        F: 'static + FnMut(&TaskContext<'_, Sid, Tid>, &RecvCtx<R>),
        R: 'static + Handoff,
    {
        let r_hid = self
            .taskpool
            .default_state::<Option<StateHandle<HandoffData<R, _>, _>>>();

        let tid = self.taskpool.new_task(move |ctx: TaskContext<'_, _, _>| {
            let r_hid = ctx.get_state_ref(r_hid).expect("Handoff not connected.");
            let recv_handoff_data = ctx.get_state_ref(r_hid);
            let recv_ctx = RefCast::ref_cast(&recv_handoff_data.handoff);

            (subgraph)(&ctx, recv_ctx);
        });

        let input_port = InputPort { subgraph: tid, handle: r_hid };
        (tid, input_port)
    }

    pub fn add_edge<H>(
        &mut self,
        output_port: OutputPort<H, Sid, Tid>,
        input_port: InputPort<H, Sid, Tid>,
    ) where
        H: 'static + Handoff,
    {
        let sid = self.taskpool.new_state(HandoffData {
            handoff: H::default(),
            pred: output_port.subgraph,
            succ: input_port.subgraph,
        });
        *self.taskpool.get_state_mut(output_port.handle) = Some(sid);
        *self.taskpool.get_state_mut(input_port.handle) = Some(sid);
    }

    pub fn tick(&mut self) {
        self.taskpool.tick()
    }

    // #[cfg(feature = "variadic_generics")]
    // #[must_use]
    // pub fn add_subgraph_stateful<F, R, W>(
    //     &mut self,
    //     mut subgraph: F,
    // ) -> (R::InputPort, W::OutputPort)
    // where
    //     F: 'static + FnMut(&TaskContext<'_, _, _>, R::RecvCtx<'_>, W::SendCtx<'_>),
    //     R: 'static + HandoffList,
    //     W: 'static + HandoffList,
    // {

    // }
}

pub(crate) struct HandoffData<H, Tid>
where
    H: Handoff,
{
    handoff: H,
    #[allow(dead_code)]
    pred: Tid,
    succ: Tid,
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_filter() {
        use std::cell::RefCell;
        use std::rc::Rc;

        use super::handoff::VecHandoff;

        // A simple dataflow with one source feeding into one sink with some processing in the middle.
        let mut df = Hydroflow::new();

        let data = [1, 2, 3, 4];
        let (_, source) = df.add_source(move |_ctx: &TaskContext<'_, _, _>, send: &SendCtx<VecHandoff<_>>| {
            for x in data.into_iter() {
                send.give(Some(x));
            }
        });

        let (_, map_in, map_out) = df.add_inout(
            |_ctx: &TaskContext<'_, _, _>, recv: &RecvCtx<VecHandoff<i32>>, send: &SendCtx<VecHandoff<_>>| {
                for x in recv.take_inner().into_iter() {
                    send.give(Some(3 * x + 1));
                }
            },
        );

        let (_, filter_in, filter_out) = df.add_inout(
            |_ctx: &TaskContext<'_, _, _>, recv: &RecvCtx<VecHandoff<i32>>, send: &SendCtx<VecHandoff<_>>| {
                for x in recv.take_inner().into_iter() {
                    if x % 2 == 0 {
                        send.give(Some(x));
                    }
                }
            },
        );

        let outputs = Rc::new(RefCell::new(Vec::new()));
        let inner_outputs = outputs.clone();
        let (_, sink) = df.add_sink(move |_ctx: &TaskContext<'_, _, _>, recv: &RecvCtx<VecHandoff<i32>>| {
            for x in recv.take_inner().into_iter() {
                (*inner_outputs).borrow_mut().push(x);
            }
        });

        df.add_edge(source, map_in);
        df.add_edge(map_out, filter_in);
        df.add_edge(filter_out, sink);

        df.tick();

        assert_eq!((*outputs).borrow().clone(), vec![4, 10]);
    }
}
