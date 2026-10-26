use crate::topology::{NodeId, RegionId};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PartitionRule {
    CompleteIsolation {
        isolated_nodes: HashSet<NodeId>,
    },
    BipartiteSplit {
        side_a: HashSet<NodeId>,
        side_b: HashSet<NodeId>,
    },
    AsymmetricDrop {
        from: NodeId,
        to: NodeId,
        drop_prob: f64,
    },
    RegionBlackout {
        region: RegionId,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NetworkPartitionMatrix {
    active_rules: Vec<PartitionRule>,
}

impl NetworkPartitionMatrix {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_rule(&mut self, rule: PartitionRule) {
        todo!()
    }

    pub fn clear_rules(&mut self) {
        todo!()
    }

    pub fn drop_probability(&self, src: NodeId, dst: NodeId) -> f64 {
        todo!()
    }
}
