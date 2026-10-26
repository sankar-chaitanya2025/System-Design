use crate::nodes::traits::HardwareResourceSpec;
use crate::time::SimDuration;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamProcessorConfig {
    pub hardware: HardwareResourceSpec,
    pub parallelism_slots: u16,
    pub window_duration: SimDuration,
    pub checkpoint_interval: SimDuration,
    pub state_backend_rocksdb: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamProcessorNode {
    pub id: NodeId,
    pub config: StreamProcessorConfig,
    pub watermark_lag_ns: u64,
    pub checkpoint_in_progress: bool,
}
