use crate::chaos::ChaosFaultSpec;
use crate::consistency::{ConsensusMessage, ReplicationDirective};
use crate::payloads::{FluidDensityWave, RequestPacket, ResponsePacket};
use crate::time::SimTime;
use crate::topology::{EdgeId, NodeId};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EventId(pub u64);

///Tie-breaking priority when two events share the exact same `SimTime`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EventPriority {
    ChaosFaultInjection = 0,
    ConsensusControl = 1,
    InternalTimer = 2,
    NetworkArrival = 3,
    RequestIngress = 4,
    FluidDensityUpdate = 5,
    TelemetryFlush = 6,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    PacketIngress {
        target_node: NodeId,
        packet: RequestPacket,
    },
    PacketTransmitStart {
        edge: EdgeId,
        packet: RequestPacket,
    },
    PacketArrival {
        edge: EdgeId,
        src_node: NodeId,
        dst_node: NodeId,
        packet: RequestPacket,
    },
    ResponseArrival {
        edge: EdgeId,
        src_node: NodeId,
        dst_node: NodeId,
        response: ResponsePacket,
    },
    NodeComputeComplete {
        node: NodeId,
        request_id: u64,
        worker_slot: usize,
    },
    DiskIoComplete {
        node: NodeId,
        op_id: u64,
        bytes_processed: usize,
    },
    FluidWavePropagation {
        edge: EdgeId,
        wave: FluidDensityWave,
    },
    ConsensusTick {
        node: NodeId,
        message: ConsensusMessage,
    },
    ReplicationSync {
        primary: NodeId,
        replica: NodeId,
        directive: ReplicationDirective,
    },
    StorageCompactionTick {
        node: NodeId,
        level: u8,
    },
    HealthCheckProbe {
        prober: NodeId,
        target: NodeId,
    },
    RequestTimeoutTrigger {
        node: NodeId,
        request_id: u64,
    },
    ChaosAction {
        fault: ChaosFaultSpec,
        activate: bool,
    },
    TelemetrySampleTick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: EventId,
    pub timestamp: SimTime,
    pub priority: EventPriority,
    pub sequence_no: u64,
    pub source_node: Option<NodeId>,
    pub target_node: Option<NodeId>,
    pub payload: EventPayload,
    pub cancelled: bool,
}

impl PartialEq for Event {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl Ord for Event {
    fn cmp(&self, _other: &Self) -> Ordering {
        todo!()
    }
}
