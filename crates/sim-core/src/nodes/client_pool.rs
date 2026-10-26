use crate::math::ArrivalRateCurve;
use crate::payloads::{ConsistencyRequirement, OperationType};
use crate::time::SimDuration;
use crate::topology::ids::{NodeId, RegionId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryPolicySpec {
    pub max_retries: u8,
    pub base_backoff: SimDuration,
    pub max_backoff: SimDuration,
    pub exponential_multiplier: f64,
    pub jitter_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientPoolConfig {
    pub simulated_user_count: u64,
    pub arrival_curve: ArrivalRateCurve,
    pub read_write_ratio: f64,
    pub key_distribution_zipf_s: f64,
    pub keyspace_cardinality: u64,
    pub request_timeout: SimDuration,
    pub retry_policy: RetryPolicySpec,
    pub default_consistency: ConsistencyRequirement,
    pub primary_operation: OperationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientPoolNode {
    pub id: NodeId,
    pub region: RegionId,
    pub config: ClientPoolConfig,
    pub active_sessions: u64,
    pub total_requests_emitted: u64,
    pub total_timeouts_observed: u64,
}