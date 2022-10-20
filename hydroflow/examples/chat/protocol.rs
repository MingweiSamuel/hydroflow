use chrono::prelude::*;
use hydroflow::util::Split;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Split)]
pub enum Message {
    ConnectRequest {
        nickname: String,
        addr: SocketAddr,
    },

    ChatMsg {
        nickname: String,
        message: String,
        ts: DateTime<Utc>,
    },

    ConnectResponse,
}
