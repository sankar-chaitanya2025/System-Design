use crate::time::SimDuration;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Parametric latency and arrival distributions for realistic tail-latency modeling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LatencyDistributionSpec {
    Constant {
        duration_ns: u64,
    },
    Uniform {
        min_ns: u64,
        max_ns: u64,
    },
    Normal {
        mean_ns: f64,
        std_dev_ns: f64,
    },
    /// Standard heavy-tail latency distribution for GC pauses and disk stalls.
    LogNormal {
        mu: f64,
        sigma: f64,
        base_offset_ns: u64,
    },
    Exponential {
        lambda: f64,
    },
    Weibull {
        shape: f64,
        scale_ns: f64,
    },
    Pareto {
        scale_ns: f64,
        alpha: f64,
    },
    /// Bimodal distribution (e.g., L1 cache hit vs cold SSTable disk read).
    Bimodal {
        fast_branch_prob: f64,
        fast_dist: Box<LatencyDistributionSpec>,
        slow_dist: Box<LatencyDistributionSpec>,
    },
}

impl LatencyDistributionSpec {
    pub fn sample<R: Rng>(&self, _rng: &mut R) -> SimDuration {
        todo!()
    }

    pub fn theoretical_mean_ns(&self) -> f64 {
        todo!()
    }

    pub fn theoretical_variance_ns2(&self) -> f64 {
        todo!()
    }

    pub fn coefficient_of_variation(&self) -> f64 {
        todo!()
    }
}
