use crate::event::EventId;
use crate::time::SimTime;
use crate::topology::{EdgeId, NodeId};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type SimResult<T> = Result<T, SimError>;

#[derive(Error, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SimError {
    #[error("Node not found in topology: {0:?}")]
    NodeNotFound(NodeId),

    #[error("Edge not found in topology: {0:?}")]
    EdgeNotFound(EdgeId),

    #[error("Topology graph cycle or invalid link between {from:?} and {to:?}: {reason}")]
    InvalidTopologyLink {
        from: NodeId,
        to: NodeId,
        reason: String,
    },

    #[error("Causality violation: event {event_id:?} scheduled at {scheduled_at:?} before current clock {current_clock:?}")]
    CausalityViolation {
        event_id: EventId,
        scheduled_at: SimTime,
        current_clock: SimTime,
    },

    #[error("Node {node_id:?} resource exhausted: {resource} (capacity: {capacity})")]
    ResourceExhausted {
        node_id: NodeId,
        resource: String,
        capacity: u64,
    },

    #[error("Quorum unavailable for group {group_id}: got {ack_count}/{required_quorum} acks")]
    QuorumUnavailable {
        group_id: String,
        ack_count: usize,
        required_quorum: usize,
    },

    #[error("Network partition blocked delivery from {src:?} to {dst:?}")]
    NetworkPartitioned { src: NodeId, dst: NodeId },

    #[error("Request {request_id} timed out at {timestamp:?}")]
    RequestTimeout {
        request_id: u64,
        timestamp: SimTime,
    },

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid engine configuration: {0}")]
    InvalidConfiguration(String),
}
