use crate::topology::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalOrdering {
    Equal,
    HappenedBefore,
    HappenedAfter,
    ConcurrentConflict,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VectorClock {
    versions: BTreeMap<NodeId, u64>,
}

impl VectorClock {
    pub fn new() -> Self {
        todo!()
    }

    pub fn increment(&mut self, node: NodeId) {
        todo!()
    }

    pub fn merge(&mut self, other: &VectorClock) {
        todo!()
    }

    pub fn compare(&self, other: &VectorClock) -> CausalOrdering {
        todo!()
    }
}
