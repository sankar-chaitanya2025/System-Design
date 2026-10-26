use crate::serialization::TopologyBlueprint;
use crate::telemetry::SloDefinition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DifficultyTier {
    BeginnerFoundations,
    IntermediateScaling,
    AdvancedDdiaConsistency,
    PrincipalMillionUserChaos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioPreset {
    pub id: String,
    pub title: String,
    pub tier: DifficultyTier,
    pub briefing_markdown: String,
    pub initial_blueprint: TopologyBlueprint,
    pub required_slos: Vec<SloDefinition>,
    pub max_monthly_budget_usd: Option<f64>,
}

impl ScenarioPreset {
    pub fn builtin_presets() -> Vec<Self> {
        todo!()
    }
}
