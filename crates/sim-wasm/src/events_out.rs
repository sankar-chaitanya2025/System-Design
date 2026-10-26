use serde::{Deserialize, Serialize};
use sim_core::engine::StepExecutionSummary;
use sim_core::payloads::TraceSpan;
use sim_core::telemetry::{ArchitecturalAnomaly, SystemGlobalMetrics};

/// High-level frame digest emitted from Wasm to the UI after a simulation tick.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmFrameDigest {
    pub step_summary: StepExecutionSummary,
    pub global_metrics: SystemGlobalMetrics,
    pub newly_detected_anomalies: Vec<ArchitecturalAnomaly>,
    pub sampled_traces: Vec<TraceSpan>,
}
