use crate::consistency::ConsensusProtocol;
use crate::time::SimDuration;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusCoordinatorConfig {
    pub protocol: ConsensusProtocol,
    pub cluster_members: Vec<NodeId>,
    pub session_lease_ttl: SimDuration,
    pub election_timeout_min: SimDuration,
    pub election_timeout_max: SimDuration,
    pub heartbeat_interval: SimDuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusCoordinatorNode {
    pub id: NodeId,
    pub config: ConsensusCoordinatorConfig,
    pub current_term: u64,
    pub current_leader: Option<NodeId>,
    pub active_ephemeral_leases: u32,
    pub committed_log_index: u64,
}
