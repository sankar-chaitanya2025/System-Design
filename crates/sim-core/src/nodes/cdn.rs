use crate::nodes::cache::EvictionPolicy;
use crate::time::SimDuration;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnEdgeConfig {
    pub storage_capacity_gb: u32,
    pub default_ttl: SimDuration,
    pub eviction_policy: EvictionPolicy,
    pub origin_shield_enabled: bool,
    pub request_collapsing_enabled: bool,
    pub origin_node: Option<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnEdgeNode {
    pub id: NodeId,
    pub config: CdnEdgeConfig,
    pub estimated_hit_ratio: f64,
    pub in_flight_origin_fetches: u32,
}
