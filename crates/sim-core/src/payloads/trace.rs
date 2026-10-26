use crate::time::SimTime;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

pub type TraceId = u128;
pub type SpanId = u64;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SpanContext {
    pub trace_id: TraceId,
    pub parent_span_id: Option<SpanId>,
    pub current_span_id: SpanId,
    pub sampled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceSpan {
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub parent_span_id: Option<SpanId>,
    pub node_id: NodeId,
    pub operation_name: String,
    pub start_time: SimTime,
    pub end_time: SimTime,
    pub queue_wait_ns: u64,
    pub service_exec_ns: u64,
    pub error_tag: Option<String>,
}
