use super::{Pull, Push};

use crate::compiled::Pusherator;
use crate::scheduled::context::Context;
use crate::scheduled::handoff::HandoffList;

pub struct SubgraphBuild<I, O>
where
    I: Pull,
    O: Push<Item = I::Item>,
{
    i: I,
    o: O,
}
impl<I, O> SubgraphBuild<I, O>
where
    I: Pull,
    O: Push<Item = I::Item>,
{
    pub(crate) fn run<'a>(
        &'a mut self,
        _context: &Context<'_>,
        recv_ctx: <I::InputHandoffs as HandoffList>::RecvCtx<'a>,
        send_ctx: <O::OutputHandoffs as HandoffList>::SendCtx<'a>,
    ) {
        let mut out = self.o.build(send_ctx);
        for x in self.i.build(recv_ctx) {
            out.give(x)
        }
    }
}
