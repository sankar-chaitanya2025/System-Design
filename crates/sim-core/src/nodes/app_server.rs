use crate::math::LatencyDistributionSpec;
use crate::nodes::traits::HardwareResourceSpec;
use crate::time::SimDuration;
use crate::topology::ids::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum ConcurrencyExecutionModel {
    OSThreadPool {
        max_worker_threads: u16,
        context_switch_overhead_ns: u64,
    },
    AsyncEventLoop {
        reactor_threads: u8,
        max_in_flight_tasks: u32,
    },
    ActorMailbox {
        mailbox_capacity: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GarbageCollectionPauseSpec {
    pub enabled: bool,
    pub allocation_rate_mb_per_req: f64,
    pub heap_size_mb: u32,
    pub stop_the_world_pause_dist: LatencyDistributionSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppServerConfig {
    pub hardware: HardwareResourceSpec,
    pub concurrency_model: ConcurrencyExecutionModel,
    pub base_compute_latency: LatencyDistributionSpec,
    pub max_listen_queue_depth: u32,
    pub downstream_connection_pool_size: u16,
    pub gc_profile: GarbageCollectionPauseSpec,
    pub auto_scaling_group_min: u16,
    pub auto_scaling_group_max: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppServerNode {
    pub id: NodeId,
    pub config: AppServerConfig,
    pub active_workers: u32,
    pub queued_requests: u32,
    pub active_replica_instances: u16,
    pub in_gc_pause_until_ns: Option<u64>,
}