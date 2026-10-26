use crate::chaos::fault::{ChaosFaultId, ChaosFaultSpec};
use crate::time::{SimDuration, SimTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosSchedule {
    pub id: ChaosFaultId,
    pub label: String,
    pub fault: ChaosFaultSpec,
    pub start_time: SimTime,
    pub duration: Option<SimDuration>,
    pub active: bool,
}
