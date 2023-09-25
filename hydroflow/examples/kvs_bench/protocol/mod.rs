mod serialization;

#[cfg(test)]
mod test;

use hydroflow_macro::Demux;
use lattices::map_union::MapUnionHashMap;
use lattices::set_union::SetUnionHashSet;
use lattices::{DomPair, Max, Point, WithBot};
pub use serialization::KvsRequestDeserializer;

use crate::buffer_pool::AutoReturnBuffer;

pub type NodeId = usize;

pub type MyLastWriteWins<const SIZE: usize> =
    DomPair<Max<u128>, WithBot<Point<AutoReturnBuffer<SIZE>, ()>>>;
pub type MySetUnion = SetUnionHashSet<(NodeId, usize)>;

#[derive(Clone, Debug)]
pub enum KvsRequest<const SIZE: usize> {
    Put {
        key: u64,
        value: AutoReturnBuffer<SIZE>,
    },
    Get {
        key: u64,
    },
    Delete {
        key: u64,
    },
    Gossip {
        map: MapUnionHashMap<u64, MyLastWriteWins<SIZE>>,
    },
}

#[derive(Clone, Debug, Demux)]
pub enum KvsRequestDemux<const SIZE: usize> {
    Put {
        tick_idx: usize,
        key: u64,
        value: AutoReturnBuffer<SIZE>,
    },
    Get {
        key: u64,
        address: NodeId,
    },
    Delete {
        tick_idx: usize,
        key: u64,
    },
    Gossip {
        map: MapUnionHashMap<u64, MyLastWriteWins<SIZE>>,
    },
}
impl<const SIZE: usize> KvsRequestDemux<SIZE> {
    pub fn from_kvs_request(req: KvsRequest<SIZE>, address: NodeId, tick_idx: usize) -> Self {
        match req {
            KvsRequest::Put { key, value } => Self::Put {
                tick_idx,
                key,
                value,
            },
            KvsRequest::Get { key } => Self::Get { key, address },
            KvsRequest::Delete { key } => Self::Delete { tick_idx, key },
            KvsRequest::Gossip { map } => Self::Gossip { map },
        }
    }
}

#[derive(Clone, Debug)]
pub enum KvsResponse<const SIZE: usize> {
    _PutResponse {
        key: u64,
    },
    GetResponse {
        key: u64,
        reg: MyLastWriteWins<SIZE>,
    },
}
