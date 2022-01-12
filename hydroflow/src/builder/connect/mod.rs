mod binary_pull;
mod binary_push;

pub use binary_pull::BinaryPullConnect;
pub use binary_push::BinaryPushConnect;

use crate::scheduled::handoff::HandoffList;

pub trait PullConnect {
    type InputHandoffs: HandoffList;
    fn connect(self, ports: <Self::InputHandoffs as HandoffList>::InputPort);
}

pub trait PushConnect {
    type OutputHandoffs: HandoffList;
    fn connect(self, ports: <Self::OutputHandoffs as HandoffList>::OutputPort);
}
