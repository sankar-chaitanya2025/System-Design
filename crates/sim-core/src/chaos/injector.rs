use crate::chaos::fault::ChaosFaultId;
use crate::chaos::schedule::ChaosSchedule;
use crate::error::SimResult;
use crate::topology::TopologyGraph;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct FaultEngine {
    schedules: HashMap<ChaosFaultId, ChaosSchedule>,
    next_id: u32,
}

impl FaultEngine {
    pub fn new() -> Self {
        todo!()
    }

    pub fn register_fault(&mut self, schedule: ChaosSchedule) -> ChaosFaultId {
        todo!()
    }

    pub fn apply_fault(&mut self, id: ChaosFaultId, topology: &mut TopologyGraph) -> SimResult<()> {
        todo!()
    }

    pub fn heal_fault(&mut self, id: ChaosFaultId, topology: &mut TopologyGraph) -> SimResult<()> {
        todo!()
    }

    pub fn active_faults(&self) -> Vec<&ChaosSchedule> {
        todo!()
    }
}
