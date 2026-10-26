use serde::{Deserialize, Serialize};
use sim_core::chaos::{ChaosFaultId, ChaosSchedule};
use sim_core::config::FidelityMode;
use sim_core::nodes::NodeKind;
use sim_core::topology::{EdgeId, NetworkEdge, NodeId, NodePlacement};

/// Mutating commands sent from React UI threads to the Wasm Simulation Engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiEngineCommand {
    LoadBlueprintJson { raw_json: String },
    UpsertNode { placement: NodePlacement, kind: NodeKind },
    RemoveNode { node_id: NodeId },
    UpsertEdge { edge: NetworkEdge },
    RemoveEdge { edge_id: EdgeId },
    SetClientTrafficRps { client_node: NodeId, target_rps: f64 },
    SetFidelityMode { mode: FidelityMode },
    ScheduleChaosFault { schedule: ChaosSchedule },
    CancelChaosFault { fault_id: ChaosFaultId },
    ResetSimulation { new_seed: Option<u64> },
}
