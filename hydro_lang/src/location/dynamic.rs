//! Definitions for interacting with locations using an untyped interface.
//!
//! Under the hood, locations are associated with a [`LocationId`] value that
//! uniquely identifies the location. Manipulating these values is useful for
//! observability and transforming the Hydro IR.

use serde::{Deserialize, Serialize};

use super::LocationKey;
#[cfg(stageleft_runtime)]
use crate::compile::{
    builder::FlowState,
    ir::{CollectionKind, HydroIrMetadata},
};
use crate::location::LocationType;

/// An ID representing a location, including "virtual" locations (atomic/tick).
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash, Serialize, Deserialize)]
pub enum LocationId {
    Process(LocationKey),
    Cluster(LocationKey),
    Atomic(
        /// The tick that the atomic region is associated with.
        Box<LocationId>,
    ),
    Tick(usize, Box<LocationId>),
}

impl LocationId {
    /// The root `LocationId` (Process or Cluster) associated with this location.
    ///
    /// For `Tick` or `Atomic`, this is the location the Tick or Atomic exists upon.
    pub fn root(&self) -> &LocationId {
        match self {
            LocationId::Process(_) => self,
            LocationId::Cluster(_) => self,
            LocationId::Atomic(tick) => tick.root(),
            LocationId::Tick(_, id) => id.root(),
        }
    }

    /// Returns true if this location is a root location (Process or Cluster).
    pub fn is_root(&self) -> bool {
        match self {
            LocationId::Process(_) | LocationId::Cluster(_) => true,
            LocationId::Atomic(_) => false,
            LocationId::Tick(_, _) => false,
        }
    }

    /// Returns true if this is a top-level location (Process, Cluster, or Atomic).
    pub fn is_top_level(&self) -> bool {
        match self {
            LocationId::Process(_) | LocationId::Cluster(_) => true,
            LocationId::Atomic(_) => true,
            LocationId::Tick(_, _) => false,
        }
    }

    /// The underlying key for root locations. Panics if this location is not a root.
    pub fn key(&self) -> LocationKey {
        match self {
            LocationId::Process(id) => *id,
            LocationId::Cluster(id) => *id,
            LocationId::Atomic(_) => panic!("cannot get raw id for atomic, use root() first"),
            LocationId::Tick(_, _) => panic!("cannot get raw id for tick, use root() first"),
        }
    }

    /// The underlying kind for root locations. Returns `None` if this location is not a root.
    pub fn location_type(&self) -> Option<LocationType> {
        match self {
            LocationId::Process(_) => Some(LocationType::Process),
            LocationId::Cluster(_) => Some(LocationType::Cluster),
            LocationId::Atomic(_) => None,
            LocationId::Tick(_, _) => None,
        }
    }

    /// Replaces the underyling root with `new_root`.
    pub fn swap_root(&mut self, new_root: LocationId) {
        match self {
            LocationId::Tick(_, id) => {
                id.swap_root(new_root);
            }
            LocationId::Atomic(tick) => {
                tick.swap_root(new_root);
            }
            _ => {
                assert!(new_root.is_root());
                *self = new_root;
            }
        }
    }
}

#[cfg(stageleft_runtime)]
pub(crate) trait DynLocation: Clone {
    fn id(&self) -> LocationId;

    fn flow_state(&self) -> &FlowState;
    fn is_top_level() -> bool;

    fn new_node_metadata(&self, collection_kind: CollectionKind) -> HydroIrMetadata {
        use crate::compile::ir::HydroIrOpMetadata;
        use crate::compile::ir::backtrace::Backtrace;

        HydroIrMetadata {
            location_id: self.id(),
            collection_kind,
            cardinality: None,
            tag: None,
            op: HydroIrOpMetadata {
                backtrace: Backtrace::get_backtrace(2),
                cpu_usage: None,
                network_recv_cpu_usage: None,
                id: None,
            },
        }
    }
}
