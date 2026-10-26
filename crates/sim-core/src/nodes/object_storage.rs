use crate::math::LatencyDistributionSpec;
use crate::time::SimDuration;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectStorageConfig {
    pub erasure_coding_data_shards: u8,
    pub erasure_coding_parity_shards: u8,
    pub time_to_first_byte: LatencyDistributionSpec,
    pub max_prefix_rps_limit: u32,
    pub lifecycle_glacier_transition: Option<SimDuration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectStorageNode {
    pub id: NodeId,
    pub config: ObjectStorageConfig,
    pub stored_objects_count: u64,
    pub total_stored_bytes: u64,
}
