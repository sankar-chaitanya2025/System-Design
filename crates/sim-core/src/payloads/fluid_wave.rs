use crate::payloads::request::{ConsistencyRequirement, OperationType};
use crate::time::SimTime;
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

/// Macro-scale statistical representation of millions of requests flowing over an edge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluidDensityWave {
    pub wave_id: u64,
    pub source_node: NodeId,
    pub timestamp: SimTime,
    pub rate_rps: f64,
    pub read_fraction: f64,
    pub write_fraction: f64,
    pub mean_payload_bytes: f64,
    pub key_skew_zipf_exponent: f64,
    pub primary_op: OperationType,
    pub consistency: ConsistencyRequirement,
}
