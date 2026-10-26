use crate::time::SimTime;
use crate::topology::{EdgeId, NodeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyCategory {
    SinglePointOfFailure,
    CacheStampedeThunderingHerd,
    RetryStormAmplification,
    SplitBrainMultipleLeaders,
    ReplicationLagSloBreach,
    HotShardPartitionSkew,
    ConnectionPoolExhaustion,
    LsmCompactionWriteStall,
    QueueConsumerBacklogExplosion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitecturalAnomaly {
    pub detected_at: SimTime,
    pub category: AnomalyCategory,
    pub affected_nodes: Vec<NodeId>,
    pub affected_edges: Vec<EdgeId>,
    pub severity_score: u8,
    pub explanation: String,
    pub remediation_hint: String,
}

pub struct BottleneckDetector;

impl BottleneckDetector {
    pub fn analyze_for_anomalies() -> Vec<ArchitecturalAnomaly> {
        todo!()
    }
}
