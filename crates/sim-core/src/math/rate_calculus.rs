use crate::time::{SimDuration, SimTime};
use serde::{Deserialize, Serialize};

/// Time-varying arrival rate profile for workload generators (diurnal curves, flash crowds, DDoS).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrivalRateCurve {
    ConstantRps {
        rps: f64,
    },
    StepFunction {
        initial_rps: f64,
        step_at: SimTime,
        target_rps: f64,
    },
    LinearRamp {
        start_rps: f64,
        end_rps: f64,
        duration: SimDuration,
    },
    SinusoidalDiurnal {
        baseline_rps: f64,
        peak_amplitude_rps: f64,
        period: SimDuration,
    },
    FlashCrowdSpike {
        baseline_rps: f64,
        spike_multiplier: f64,
        onset_time: SimTime,
        decay_half_life: SimDuration,
    },
}

impl ArrivalRateCurve {
    pub fn evaluate_rps_at(&self, _time: SimTime) -> f64 {
        todo!()
    }
}

pub struct LittleLawSolver;

impl LittleLawSolver {
    /// `L = lambda * W`: Calculates concurrent in-flight connections or threads required.
    pub fn concurrency_l(arrival_rate_rps: f64, mean_latency: SimDuration) -> f64 {
        todo!()
    }
}
