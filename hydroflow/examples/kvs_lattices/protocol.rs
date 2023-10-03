use std::net::SocketAddr;

use hydroflow_macro::DemuxEnum;
use lattices::map_union::{MapUnionHashMap, MapUnionSingletonMap};
use lattices::{DomPair, Max};
use serde::{Deserialize, Serialize};

pub type KvsValue = DomPair<VecClock, Max<String>>;
pub type VecClock = MapUnionSingletonMap<u32, usize>;

#[derive(Clone, Debug, Serialize, Deserialize, DemuxEnum)]
pub enum KvsMessage {
    Put { key: String, value: KvsValue },
    Get { key: String },
}

#[derive(Clone, Debug, DemuxEnum)]
pub enum KvsMessageWithAddr {
    Put {
        key: String,
        value: KvsValue,
        addr: SocketAddr,
    },
    Get {
        key: String,
        addr: SocketAddr,
    },
}
impl KvsMessageWithAddr {
    pub fn from_message(message: KvsMessage, addr: SocketAddr) -> Self {
        match message {
            KvsMessage::Put { key, value } => Self::Put { key, value, addr },
            KvsMessage::Get { key } => Self::Get { key, addr },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KvsResponse {
    pub key: String,
    pub value: KvsValue,
}
