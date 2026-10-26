use crate::nodes::traits::HardwareResourceSpec;
use crate::time::SimDuration;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryGuarantee {
    AtMostOnce,
    AtLeastOnce { ack_timeout: SimDuration },
    ExactlyOnceTransactional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackpressurePolicy {
    BlockProducer,
    DropOldestHead,
    RejectNewestTail,
    SpillToDiskLog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageQueueConfig {
    pub hardware: HardwareResourceSpec,
    pub partition_count: u16,
    pub replication_factor: u8,
    pub delivery_guarantee: DeliveryGuarantee,
    pub backpressure_policy: BackpressurePolicy,
    pub retention_storage_gb: u32,
    pub max_batch_bytes: u32,
    pub linger_batch_delay: SimDuration,
    pub dead_letter_queue_node: Option<NodeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageQueueNode {
    pub id: NodeId,
    pub config: MessageQueueConfig,
    pub consumer_groups: Vec<NodeId>,
    pub total_unacked_messages: u64,
    pub consumer_lag_messages: u64,
    pub dlq_routed_messages: u64,
}
