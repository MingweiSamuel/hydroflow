use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::Rc;

use sealed::sealed;

use crate::scheduled::ctx::{HandoffStateHandle, InputPort, OutputPort, RecvCtx, SendCtx};
use crate::scheduled::handoff::Handoff;
use crate::scheduled::state::StateHandle;
use crate::scheduled::{Context, SubgraphId};

/**
 * A variadic list of Handoff types, represented using a lisp-style tuple structure.
 *
 * This trait is sealed and not meant to be implemented or used directly. Instead tuple lists (which already implement this trait) should be used, for example:
 * ```ignore
 * type MyHandoffList = (VecHandoff<usize>, (VecHandoff<String>, (TeeingHandoff<u32>, ())));
 * ```
 * The [`tl!`] (tuple list) macro simplifies usage of this kind:
 * ```ignore
 * type MyHandoffList = tl!(VecHandoff<usize>, VecHandoff<String>, TeeingHandoff<u32>);
 * ```
 */
#[sealed]
pub trait HandoffList {
    type InputHid;
    type InputPort;
    type RecvCtx<'a>;
    fn make_input(sg_id: SubgraphId) -> (Self::InputHid, Self::InputPort);
    fn make_recv<'a>(context: &'a Context<'a>, input_hids: &Self::InputHid) -> Self::RecvCtx<'a>;

    type OutputHid;
    type OutputPort;
    type SendCtx<'a>;
    fn make_output(sg_id: SubgraphId) -> (Self::OutputHid, Self::OutputPort);
    fn make_send<'a>(context: &'a Context<'a>, output_hids: &Self::OutputHid) -> Self::SendCtx<'a>;
}
#[sealed]
impl<H, L> HandoffList for (H, L)
where
    H: 'static + Handoff,
    L: HandoffList,
{
    #[allow(clippy::type_complexity)]
    type InputHid = (Rc<Cell<Option<HandoffStateHandle<H>>>>, L::InputHid);
    type InputPort = (InputPort<H>, L::InputPort);
    type RecvCtx<'a> = (RecvCtx<'a, H>, L::RecvCtx<'a>);
    fn make_input(sg_id: SubgraphId) -> (Self::InputHid, Self::InputPort) {
        let hid = <Rc<Cell<Option<StateHandle<RefCell<H::State>>>>>>::default();
        let input = InputPort {
            sg_id,
            state_handle: hid.clone(),
            _phantom: PhantomData,
        };

        let (hid_rest, input_rest) = L::make_input(sg_id);

        ((hid, hid_rest), (input, input_rest))
    }
    fn make_recv<'a>(context: &'a Context<'a>, input_hids: &Self::InputHid) -> Self::RecvCtx<'a> {
        let (hid, hid_rest) = input_hids;
        let hid = hid.get().expect("Attempted to use unattached handoff.");

        let ctx = RecvCtx {
            state_handle: hid,
            context,
            _phantom: PhantomData,
        };

        let ctx_rest = L::make_recv(context, hid_rest);
        (ctx, ctx_rest)
    }

    #[allow(clippy::type_complexity)]
    type OutputHid = (Rc<Cell<Option<HandoffStateHandle<H>>>>, L::OutputHid);
    type OutputPort = (OutputPort<H>, L::OutputPort);
    type SendCtx<'a> = (SendCtx<'a, H>, L::SendCtx<'a>);
    fn make_output(sg_id: SubgraphId) -> (Self::OutputHid, Self::OutputPort) {
        let hid = <Rc<Cell<Option<StateHandle<RefCell<H::State>>>>>>::default();
        let output = OutputPort {
            sg_id,
            state_handle: hid.clone(),
            _phantom: PhantomData,
        };

        let (hid_rest, output_rest) = L::make_output(sg_id);

        ((hid, hid_rest), (output, output_rest))
    }
    fn make_send<'a>(context: &'a Context<'a>, output_hids: &Self::OutputHid) -> Self::SendCtx<'a> {
        let (hid, hid_rest) = output_hids;
        let hid = hid.get().expect("Attempted to use unattached handoff.");

        let ctx = SendCtx {
            state_handle: hid,
            context,
            _phantom: PhantomData,
        };

        let ctx_rest = L::make_send(context, hid_rest);
        (ctx, ctx_rest)
    }
}
#[sealed]
impl HandoffList for () {
    type InputHid = ();
    type InputPort = ();
    type RecvCtx<'a> = ();
    fn make_input(_: SubgraphId) -> (Self::InputHid, Self::InputPort) {
        ((), ())
    }
    fn make_recv<'a>(
        _handoffs: &'a Context<'a>,
        _input_hids: &Self::InputHid,
    ) -> Self::RecvCtx<'a> {
    }

    type OutputHid = ();
    type OutputPort = ();
    type SendCtx<'a> = ();
    fn make_output(_: SubgraphId) -> (Self::OutputHid, Self::OutputPort) {
        ((), ())
    }
    fn make_send<'a>(
        _handoffs: &'a Context<'a>,
        _output_hids: &Self::OutputHid,
    ) -> Self::SendCtx<'a> {
    }
}
