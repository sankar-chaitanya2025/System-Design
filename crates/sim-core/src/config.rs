use crate::time::SimDuration;
use serde::{Deserialize, Serialize};

/// Determines how the engine models traffic as user scale grows from 1 to 1,000,000+ users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FidelityMode {
    /// Every request is a distinct event in the binary heap (`< 10,000 RPS`).
    MicroDiscretePacket,
    /// Analytical fluid-flow differential equations (`> 100,000 RPS`).
    MacroFluidDensity,
    /// Automatically switches node/edge processing between discrete packets and fluid flows based on threshold.
    HybridAdaptive { packet_to_fluid_threshold_rps: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub random_seed: u64,
    pub fidelity_mode: FidelityMode,
    pub fluid_integration_step: SimDuration,
    pub telemetry_window_interval: SimDuration,
    pub max_events_per_frame: usize,
    pub max_in_flight_packets: usize,
    pub enable_distributed_tracing: bool,
    pub trace_sampling_rate: f64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        todo!()
    }
}

impl SimulationConfig {
    pub fn validate(&self) -> Result<(), String> {
        todo!()
    }
}
