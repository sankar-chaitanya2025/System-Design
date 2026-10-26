use crate::time::SimDuration;
use serde::{Deserialize, Serialize};

/// Determines how the engine models traffic as user scale grows from 1 to 1,000,000+ users.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all_fields = "camelCase")]
pub enum FidelityMode {
    /// Every request is a distinct event in the binary heap (`< 10,000 RPS`).
    MicroDiscretePacket,
    /// Analytical fluid-flow differential equations (`> 100,000 RPS`).
    MacroFluidDensity,
    /// Automatically switches node/edge processing between discrete packets and fluid flows based on threshold.
    HybridAdaptive { packet_to_fluid_threshold_rps: u64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulationConfig {
    pub random_seed: u64,
    pub fidelity_mode: FidelityMode,
    pub fluid_integration_step: SimDuration,
    pub telemetry_window_interval: SimDuration,
    /// Upper bound on discrete events processed per `advance_*` call, so a single UI
    /// frame can never stall on an event storm.
    pub max_events_per_frame: usize,
    pub max_in_flight_packets: usize,
    pub enable_distributed_tracing: bool,
    pub trace_sampling_rate: f64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        SimulationConfig {
            random_seed: 42,
            fidelity_mode: FidelityMode::MicroDiscretePacket,
            fluid_integration_step: SimDuration::from_ms(1),
            telemetry_window_interval: SimDuration::from_ms(250),
            max_events_per_frame: 200_000,
            max_in_flight_packets: 1_000_000,
            enable_distributed_tracing: false,
            trace_sampling_rate: 0.01,
        }
    }
}

impl SimulationConfig {
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0..=1.0).contains(&self.trace_sampling_rate) {
            return Err(format!(
                "traceSamplingRate must be within [0, 1], got {}",
                self.trace_sampling_rate
            ));
        }
        if self.fluid_integration_step == SimDuration::ZERO {
            return Err("fluidIntegrationStep must be greater than zero".into());
        }
        if self.telemetry_window_interval == SimDuration::ZERO {
            return Err("telemetryWindowInterval must be greater than zero".into());
        }
        if self.max_events_per_frame == 0 {
            return Err("maxEventsPerFrame must be greater than zero".into());
        }
        if self.max_in_flight_packets == 0 {
            return Err("maxInFlightPackets must be greater than zero".into());
        }
        if let FidelityMode::HybridAdaptive {
            packet_to_fluid_threshold_rps: 0,
        } = self.fidelity_mode
        {
            return Err("packetToFluidThresholdRps must be greater than zero".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        assert_eq!(SimulationConfig::default().validate(), Ok(()));
    }

    #[test]
    fn invalid_fields_are_rejected() {
        let base = SimulationConfig::default();

        let mut c = base.clone();
        c.trace_sampling_rate = 1.5;
        assert!(c.validate().is_err());

        let mut c = base.clone();
        c.trace_sampling_rate = f64::NAN;
        assert!(c.validate().is_err());

        let mut c = base.clone();
        c.max_events_per_frame = 0;
        assert!(c.validate().is_err());

        let mut c = base.clone();
        c.telemetry_window_interval = SimDuration::ZERO;
        assert!(c.validate().is_err());

        let mut c = base;
        c.fidelity_mode = FidelityMode::HybridAdaptive {
            packet_to_fluid_threshold_rps: 0,
        };
        assert!(c.validate().is_err());
    }

    #[test]
    fn serializes_with_adr_0001_conventions() {
        let json = serde_json::to_value(SimulationConfig::default()).unwrap();
        assert_eq!(json["randomSeed"], 42);
        assert_eq!(json["fidelityMode"]["type"], "MicroDiscretePacket");

        let hybrid = FidelityMode::HybridAdaptive {
            packet_to_fluid_threshold_rps: 5_000,
        };
        let json = serde_json::to_value(hybrid).unwrap();
        assert_eq!(json["type"], "HybridAdaptive");
        assert_eq!(json["packetToFluidThresholdRps"], 5_000);
        let back: FidelityMode = serde_json::from_value(json).unwrap();
        assert_eq!(back, hybrid);
    }
}
