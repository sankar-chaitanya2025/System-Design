use crate::chaos::ChaosSchedule;
use crate::config::SimulationConfig;
use crate::nodes::NodeKind;
use crate::telemetry::SloDefinition;
use crate::topology::{NetworkEdge, NodePlacement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopologyBlueprint {
    pub schema_version: u16,
    pub title: String,
    pub description: String,
    pub author: String,
    pub config: SimulationConfig,
    pub placements: Vec<NodePlacement>,
    pub nodes: Vec<NodeKind>,
    pub edges: Vec<NetworkEdge>,
    pub preset_chaos_schedules: Vec<ChaosSchedule>,
    pub target_slos: Vec<SloDefinition>,
}

impl TopologyBlueprint {
    pub fn to_json_pretty(&self) -> Result<String, String> {
        todo!()
    }

    pub fn from_json_str(raw: &str) -> Result<Self, String> {
        todo!()
    }
}
