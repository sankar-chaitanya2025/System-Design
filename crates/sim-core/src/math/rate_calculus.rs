use serde::{Deserialize, Serialize};
use std::f64::consts::TAU;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum WorkloadCurve {
    Constant { rps: f64 },
    Sinusoidal { base_rps: f64, amplitude: f64, period_secs: f64 },
    FlashCrowd { base_rps: f64, peak_rps: f64, peak_time: f64, ramp_width: f64 },
}

impl WorkloadCurve {
    pub fn current_rate(&self, time_secs: f64) -> f64 {
        match self {
            Self::Constant { rps } => *rps,
            Self::Sinusoidal { base_rps, amplitude, period_secs } => {
                let frequency = TAU / period_secs;
                base_rps + amplitude * (time_secs * frequency).sin()
            }
            Self::FlashCrowd { base_rps, peak_rps, peak_time, ramp_width } => {
                let distance = (time_secs - peak_time).abs();
                if distance > *ramp_width {
                    *base_rps
                } else {
                    let scale = 1.0 - (distance / ramp_width);
                    base_rps + (peak_rps - base_rps) * scale
                }
            }
        }
    }
}

pub struct LittlesLaw;

impl LittlesLaw {
    pub fn calculate_l(lambda: f64, w: f64) -> f64 {
        lambda * w
    }

    pub fn calculate_lambda(l: f64, w: f64) -> f64 {
        l / w
    }

    pub fn calculate_w(l: f64, lambda: f64) -> f64 {
        l / lambda
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sinusoidal_curve_peaks() {
        let curve = WorkloadCurve::Sinusoidal { base_rps: 100.0, amplitude: 50.0, period_secs: 10.0 };
        assert!((curve.current_rate(0.0) - 100.0).abs() < 1e-6);
        assert!((curve.current_rate(2.5) - 150.0).abs() < 1e-6);
        assert!((curve.current_rate(7.5) - 50.0).abs() < 1e-6);
    }

    #[test]
    fn test_littles_law_solver() {
        assert_eq!(LittlesLaw::calculate_l(2.0, 5.0), 10.0);
        assert_eq!(LittlesLaw::calculate_lambda(10.0, 5.0), 2.0);
        assert_eq!(LittlesLaw::calculate_w(10.0, 2.0), 5.0);
    }
}
pub type ArrivalRateCurve = WorkloadCurve;
pub type LittleLawSolver = LittlesLaw;
