use crate::config::SimulationConfig;
use crate::serialization::blueprint::TopologyBlueprint;
use crate::time::SimTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineSnapshot {
    pub captured_at: SimTime,
    pub config: SimulationConfig,
    pub blueprint: TopologyBlueprint,
    pub serialized_rng_state: Vec<u8>,
}
