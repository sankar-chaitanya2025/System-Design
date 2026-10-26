use serde::{Deserialize, Serialize};

/// Dynamo-style quorum parameters (`N` replicas, `W` write acks, `R` read responses).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TunableQuorumConfig {
    pub replication_factor_n: u8,
    pub write_quorum_w: u8,
    pub read_quorum_r: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuorumEvaluation {
    pub is_strict_quorum: bool,
    pub stale_read_possible: bool,
    pub concurrent_write_conflict_possible: bool,
    pub fault_tolerance_max_node_failures_writes: u8,
    pub fault_tolerance_max_node_failures_reads: u8,
}

impl TunableQuorumConfig {
    pub fn evaluate_guarantees(&self) -> QuorumEvaluation {
        todo!()
    }
}
