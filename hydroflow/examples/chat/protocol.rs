use std::net::SocketAddr;

use chrono::prelude::*;
use hydroflow_macro::Demux;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Demux)]
pub enum Message {
    ConnectRequest,
    ConnectResponse,
    ChatMsg {
        nickname: String,
        message: String,
        ts: DateTime<Utc>,
    },
}

#[derive(Clone, Debug, Demux)]
pub enum MessageWithAddr {
    ConnectRequest {
        addr: SocketAddr,
    },
    ConnectResponse {
        addr: SocketAddr,
    },
    ChatMsg {
        addr: SocketAddr,
        nickname: String,
        message: String,
        ts: DateTime<Utc>,
    },
}
impl MessageWithAddr {
    pub fn from_message(message: Message, addr: SocketAddr) -> Self {
        match message {
            Message::ConnectRequest => Self::ConnectRequest { addr },
            Message::ConnectResponse => Self::ConnectResponse { addr },
            Message::ChatMsg {
                nickname,
                message,
                ts,
            } => Self::ChatMsg {
                addr,
                nickname,
                message,
                ts,
            },
        }
    }
}
