use super::{Pull, Push, PushBuild};

use crate::compiled::Pusherator;
use crate::scheduled::context::Context;
use crate::scheduled::handoff::HandoffList;

pub struct Pivot<I, O>
where
    I: Pull,
    O: Push<Item = I::Item>,
{
    i: I,
    o: O,
}
impl<I, O> Pivot<I, O>
where
    I: Pull,
    O: Push<Item = I::Item>,
{
    pub fn new(i: I, o: O) -> Self {
        Pivot { i, o }
    }

    pub fn init(
        &mut self,
        input_ports: <I::InputHandoffs as HandoffList>::InputPort,
        outupt_ports: <O::OutputHandoffs as HandoffList>::OutputPort,
    ) {
        self.i.init(input_ports);
        self.o.init(outupt_ports);
    }

    pub fn run<'a>(
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

pub struct PivotBuild<I>
where
    I: Pull,
{
    pull: I,
}
impl<I> PivotBuild<I>
where
    I: Pull,
{
    pub(crate) fn new(pull: I) -> Self {
        Self { pull }
    }
}
impl<I> PushBuild for PivotBuild<I>
where
    I: Pull,
{
    type Item = I::Item;

    type Output<O>
    where
        O: Push<Item = Self::Item>,
    = Pivot<I, O>;
    fn build<O>(self, input: O) -> Self::Output<O>
    where
        O: Push<Item = Self::Item>,
    {
        Pivot::new(self.pull, input)
    }
}
