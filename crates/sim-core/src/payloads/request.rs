use crate::consistency::VectorClock;
use crate::payloads::trace::SpanContext;
use crate::time::SimTime;
use crate::topology::{NodeId, ShardId};
use serde::{Deserialize, Serialize};

pub type RequestId = u64;
pub type PayloadSizeBytes = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationType {
    ReadKey,
    WriteKey,
    RangeScan { item_count: u32 },
    CompareAndSwap,
    ComputeIntensive { cpu_instructions: u64 },
    PublishEvent,
    BlobTransfer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsistencyRequirement {
    Eventual,
    ReadYourWrites,
    MonotonicReads,
    Quorum,
    Linearizable,
    SerializableTransaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseStatus {
    Ok200,
    Created201,
    BadRequest400,
    TooManyRequests429,
    InternalServerError500,
    BadGateway502,
    ServiceUnavailable503,
    GatewayTimeout504,
    StaleReadDetected,
    WriteConflictAborted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestPacket {
    pub id: RequestId,
    pub client_origin: NodeId,
    pub created_at: SimTime,
    pub deadline_at: SimTime,
    pub routing_key_hash: u64,
    pub target_shard: Option<ShardId>,
    pub op_type: OperationType,
    pub consistency: ConsistencyRequirement,
    pub payload_bytes: PayloadSizeBytes,
    pub retry_attempt: u8,
    pub causal_version: Option<VectorClock>,
    pub trace_context: Option<SpanContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsePacket {
    pub request_id: RequestId,
    pub responder: NodeId,
    pub completed_at: SimTime,
    pub status: ResponseStatus,
    pub payload_bytes: PayloadSizeBytes,
    pub observed_version: Option<VectorClock>,
    pub is_stale_read: bool,
    pub cache_hit: bool,
}
