use crate::math::LatencyDistributionSpec;
use crate::nodes::traits::HardwareResourceSpec;
use crate::time::SimDuration;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvictionPolicy {
    Lru,
    Lfu,
    Wtinylfu,
    Fifo,
    Random,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheWritePattern {
    CacheAside,
    ReadThrough,
    WriteThrough,
    WriteBehindAsync { batch_flush_interval: SimDuration },
    WriteAround,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StampedeMitigationConfig {
    pub mutex_singleflight_lock: bool,
    pub probabilistic_early_expiration_beta: Option<f64>,
    pub stale_while_revalidate_window: Option<SimDuration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheNodeConfig {
    pub hardware: HardwareResourceSpec,
    pub max_memory_gb: f64,
    pub eviction_policy: EvictionPolicy,
    pub write_pattern: CacheWritePattern,
    pub default_ttl: SimDuration,
    pub ttl_jitter_ratio: f64,
    pub hit_latency: LatencyDistributionSpec,
    pub stampede_mitigation: StampedeMitigationConfig,
    pub backing_datastore: Option<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheNode {
    pub id: NodeId,
    pub config: CacheNodeConfig,
    pub working_set_occupancy_ratio: f64,
    pub instantaneous_hit_rate: f64,
    pub evictions_per_sec: f64,
    pub active_singleflight_locks: u32,
}
