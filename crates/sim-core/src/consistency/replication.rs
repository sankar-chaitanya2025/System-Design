use crate::consistency::vector_clock::VectorClock;
use crate::time::SimDuration;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    LastWriteWinsTimestamp,
    VectorClockSiblingMerge,
    CrdtStateConvergent,
    CustomApplicationResolver,
}

/// DDIA Chapter 5 Replication Topologies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationMode {
    SingleLeader {
        synchronous_followers: u8,
        asynchronous_followers: u8,
        semi_sync_timeout: SimDuration,
    },
    MultiLeader {
        leader_nodes: Vec<NodeId>,
        conflict_resolution: ConflictResolutionStrategy,
        inter_dc_replication_delay: SimDuration,
    },
    LeaderlessDynamo {
        sloppy_quorum_enabled: bool,
        hinted_handoff_enabled: bool,
        read_repair_probability: f64,
        anti_entropy_merkle_interval: SimDuration,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationDirective {
    pub log_sequence_number: u64,
    pub key_hash: u64,
    pub payload_bytes: u32,
    pub version: VectorClock,
    pub require_sync_ack: bool,
}
