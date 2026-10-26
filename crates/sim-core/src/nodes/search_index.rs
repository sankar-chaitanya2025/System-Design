use crate::nodes::traits::HardwareResourceSpec;
use crate::time::SimDuration;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndexConfig {
    pub hardware: HardwareResourceSpec,
    pub shard_count: u16,
    pub replica_count: u8,
    pub refresh_interval: SimDuration,
    pub segment_merge_throttling_mb_s: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchIndexNode {
    pub id: NodeId,
    pub config: SearchIndexConfig,
    pub indexed_document_count: u64,
    pub pending_refresh_docs: u64,
}
