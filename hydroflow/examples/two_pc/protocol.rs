use hydroflow_macro::Demux;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Hash, Copy, Demux)]
pub enum Message {
    Prepare(u16),
    Commit(u16),
    Abort(u16),
    AckP2(u16),
    End(u16),
    Ended(u16),
    Err(u16),
}

// #[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Hash, Copy)]
// pub enum MsgType {
//     Prepare,
//     Commit,
//     Abort,
//     AckP2,
//     End,
//     Ended,
//     Err,
// }

// #[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Demux)]
// pub struct CoordMsg {
//     pub xid: u16,
//     pub mtype: MsgType,
// }
// /// Member Response
// #[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
// pub struct SubordResponse {
//     pub xid: u16,
//     pub mtype: MsgType,
// }
