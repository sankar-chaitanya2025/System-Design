use rand::Rng;
use rand_distr::{Distribution, Exp, LogNormal, Normal, Pareto, Uniform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LatencyDistributionSpec {
    Constant { value: f64 },
    Exponential { lambda: f64 },
    Normal { mean: f64, std_dev: f64 },
    LogNormal { mu: f64, sigma: f64 },
    Uniform { min: f64, max: f64 },
    Pareto { scale: f64, shape: f64 },
}

impl LatencyDistributionSpec {
    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        match self {
            Self::Constant { value } => *value,
            Self::Exponential { lambda } => {
                let exp = Exp::new(*lambda).unwrap();
                exp.sample(rng)
            }
            Self::Normal { mean, std_dev } => {
                let normal = Normal::new(*mean, *std_dev).unwrap();
                normal.sample(rng).max(0.0) // Network latency cannot be negative
            }
            Self::LogNormal { mu, sigma } => {
                let log_normal = LogNormal::new(*mu, *sigma).unwrap();
                log_normal.sample(rng)
            }
            Self::Uniform { min, max } => {
                let uniform = Uniform::new(*min, *max);
                uniform.sample(rng)
            }
            Self::Pareto { scale, shape } => {
                let pareto = Pareto::new(*scale, *shape).unwrap();
                pareto.sample(rng)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn test_exponential_empirical_mean() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let lambda = 10.0;
        let dist = LatencyDistributionSpec::Exponential { lambda };
        
        let mut sum = 0.0;
        let samples = 1_000_000;
        
        for _ in 0..samples {
            sum += dist.sample(&mut rng);
        }
        
        let empirical_mean = sum / samples as f64;
        let theoretical_mean = 1.0 / lambda;
        
        let error = (empirical_mean - theoretical_mean).abs() / theoretical_mean;
        assert!(error < 0.01, "Empirical mean error exceeds 1% margin");
    }
}