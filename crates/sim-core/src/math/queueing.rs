use crate::time::SimDuration;
use serde::{Deserialize, Serialize};

/// Analytical M/M/c/K queueing model used when simulating high-volume traffic in MacroFluidDensity mode.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MmcQueueModel {
    pub arrival_rate_lambda: f64,
    pub service_rate_mu: f64,
    pub servers_c: usize,
    pub max_queue_capacity_k: Option<usize>,
}

/// Kingman's G/G/c formula approximation for general arrival and service distributions.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct KingmanApproximation {
    pub arrival_rate_lambda: f64,
    pub service_rate_mu: f64,
    pub servers_c: usize,
    pub coeff_var_arrival_ca: f64,
    pub coeff_var_service_cs: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QueueSteadyStateMetrics {
    pub utilization_rho: f64,
    pub erlang_c_wait_prob: f64,
    pub mean_queue_length_lq: f64,
    pub mean_system_items_l: f64,
    pub mean_wait_time_wq: SimDuration,
    pub mean_response_time_w: SimDuration,
    pub rejection_probability_pb: f64,
}

impl MmcQueueModel {
    pub fn solve_steady_state(&self) -> QueueSteadyStateMetrics {
        todo!()
    }

    pub fn erlang_c(&self) -> f64 {
        todo!()
    }

    pub fn erlang_b_loss(&self, capacity_k: usize) -> f64 {
        todo!()
    }
}

impl KingmanApproximation {
    pub fn estimate_mean_wait_time(&self) -> SimDuration {
        todo!()
    }

    pub fn estimate_tail_percentile(&self, percentile: f64) -> SimDuration {
        todo!()
    }
}
