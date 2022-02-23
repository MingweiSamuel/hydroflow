use std::collections::HashMap;

use slotmap::{new_key_type, SecondaryMap, SlotMap};

use crate::scheduled::{HandoffId, SubgraphId};

new_key_type! {
    pub struct VertexKey;
}

pub struct HydroflowMeta {
    pub vertices: SlotMap<VertexKey, VertexMeta>,
    pub subgraphs: SecondaryMap<SubgraphId, SubgraphMeta>,
}

pub struct SubgraphMeta {
    pub name: Cow<'static, str>,
}

pub struct VertexMeta {
    pub vertex_type: VertexType,
    pub succs: Vec<VertexKey>,
}

pub enum VertexType {
    Handoff {
        id: HandoffId,
        name: Cow<'static, str>,
    },
    Port {
        parent: SubgraphId,
    },
    SubgraphVertex {
        parent: SubgraphId,
    },
}
