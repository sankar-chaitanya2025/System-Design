use crate::scenarios::catalog::ScenarioPreset;
use crate::telemetry::SystemGlobalMetrics;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradingRubric {
    pub min_availability_pct: f64,
    pub max_p99_latency_ms: f64,
    pub max_budget_usd: f64,
    pub must_survive_single_node_crash: bool,
    pub must_survive_az_partition: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioEvaluationResult {
    pub passed: bool,
    pub score_out_of_100: u8,
    pub feedback_items: Vec<String>,
}

pub struct ScenarioValidator;

impl ScenarioValidator {
    pub fn evaluate(
        preset: &ScenarioPreset,
        final_metrics: &SystemGlobalMetrics,
    ) -> ScenarioEvaluationResult {
        todo!()
    }
}
