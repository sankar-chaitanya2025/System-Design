use crate::nodes::traits::HardwareResourceSpec;
use crate::time::SimDuration;
use crate::topology::ids::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LoadBalancingLayer {
    Layer4TcpPassthrough,
    Layer7HttpReverseProxy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    LeastResponseTime,
    PowerOfTwoChoices,
    ConsistentHashingRing { virtual_nodes_per_backend: u16 },
    MaglevHashTable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckConfig {
    pub interval: SimDuration,
    pub timeout: SimDuration,
    pub healthy_threshold: u8,
    pub unhealthy_threshold: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerConfig {
    pub layer: LoadBalancingLayer,
    pub algorithm: LoadBalancingAlgorithm,
    pub hardware: HardwareResourceSpec,
    pub health_check: HealthCheckConfig,
    pub max_syn_backlog: u32,
    pub connection_draining_timeout: SimDuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadBalancerNode {
    pub id: NodeId,
    pub config: LoadBalancerConfig,
    pub backends: Vec<NodeId>,
    pub active_conn_counts: HashMap<NodeId, u32>,
    pub backend_health: HashMap<NodeId, bool>,
    pub rr_cursor: usize,
}

impl LoadBalancerNode {
    pub fn select_backend(&mut self, _routing_key_hash: u64) -> Option<NodeId> {
        if self.backends.is_empty() {
            return None;
        }

        match self.config.algorithm {
            LoadBalancingAlgorithm::RoundRobin => {
                let idx = self.rr_cursor % self.backends.len();
                self.rr_cursor = self.rr_cursor.wrapping_add(1);
                Some(self.backends[idx])
            }
            LoadBalancingAlgorithm::LeastConnections => {
                self.backends
                    .iter()
                    .min_by_key(|&id| self.active_conn_counts.get(id).unwrap_or(&0))
                    .copied()
            }
            _ => {
                // Fallback for deferred algorithms in Phase 2
                let idx = self.rr_cursor % self.backends.len();
                self.rr_cursor = self.rr_cursor.wrapping_add(1);
                Some(self.backends[idx])
            }
        }
    }
}