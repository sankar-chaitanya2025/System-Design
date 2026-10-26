use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SloDefinition {
    pub name: String,
    pub target_availability_pct: f64,
    pub max_p95_latency_ms: f64,
    pub max_p99_latency_ms: f64,
    pub max_stale_read_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SloComplianceReport {
    pub slo_met: bool,
    pub error_budget_remaining_pct: f64,
    pub burn_rate_multiplier: f64,
    pub violation_reasons: Vec<String>,
}
