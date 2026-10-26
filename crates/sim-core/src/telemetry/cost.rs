use crate::topology::TopologyGraph;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBreakdown {
    pub compute_hourly_usd: f64,
    pub storage_monthly_usd: f64,
    pub network_egress_hourly_usd: f64,
    pub managed_service_overhead_usd: f64,
    pub total_monthly_estimate_usd: f64,
}

pub struct CloudCostEstimator;

impl CloudCostEstimator {
    pub fn estimate_topology_cost(
        topology: &TopologyGraph,
        avg_egress_gbps: f64,
    ) -> CostBreakdown {
        todo!()
    }
}
