mod pull_binary;
mod pull_handoff;
mod push_binary;
mod push_handoff;

pub use pull_binary::BinaryPullConnect;
pub use pull_handoff::HandoffPullConnect;
pub use push_binary::BinaryPushConnect;
pub use push_handoff::HandoffPushConnect;

use crate::scheduled::handoff::HandoffList;

pub trait PullConnect {
    type InputHandoffs: HandoffList;
    fn connect(self, ports: <Self::InputHandoffs as HandoffList>::InputPort);
}

pub trait PushConnect {
    type OutputHandoffs: HandoffList;
    fn connect(self, ports: <Self::OutputHandoffs as HandoffList>::OutputPort);
}
