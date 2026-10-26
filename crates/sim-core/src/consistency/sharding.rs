use crate::topology::{NodeId, ShardId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// DDIA Chapter 6 Partitioning Strategies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PartitioningScheme {
    UnpartitionedSingleShard,
    KeyRangePartitioning {
        dynamic_range_splitting: bool,
        hotspot_auto_split_threshold_rps: f64,
    },
    ConsistentHashPartitioning {
        virtual_vnodes_per_node: u16,
    },
    HashModulusFixed {
        num_buckets: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistentHashRing {
    ring: BTreeMap<u64, (ShardId, NodeId)>,
    vnodes_per_node: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardRebalancePlan {
    pub moving_shards: Vec<(ShardId, NodeId, NodeId)>,
    pub estimated_bytes_to_transfer: u64,
}

impl ConsistentHashRing {
    pub fn new(vnodes_per_node: u16) -> Self {
        todo!()
    }

    pub fn insert_node(&mut self, node_id: NodeId, shard_id: ShardId) {
        todo!()
    }

    pub fn remove_node(&mut self, node_id: NodeId) -> ShardRebalancePlan {
        todo!()
    }

    pub fn route_key(&self, key_hash: u64) -> Option<(ShardId, NodeId)> {
        todo!()
    }
}
