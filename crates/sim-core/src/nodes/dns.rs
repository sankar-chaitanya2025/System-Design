use crate::time::SimDuration;
use crate::topology::{NodeId, RegionId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsRoutingPolicy {
    WeightedRoundRobin,
    LatencyBasedGeoProximity,
    FailoverPrimarySecondary,
    AnycastBgp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsNode {
    pub id: NodeId,
    pub routing_policy: DnsRoutingPolicy,
    pub record_ttl: SimDuration,
    pub region_targets: HashMap<RegionId, Vec<NodeId>>,
    pub unhealthy_targets: Vec<NodeId>,
}
