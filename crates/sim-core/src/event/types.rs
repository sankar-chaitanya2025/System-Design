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
    /// Generic timer callback (GC pause end, retry backoff, autoscaler tick, test harnesses).
    TimerFired {
        node: Option<NodeId>,
        timer_id: u64,
    },
    TelemetrySampleTick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Assigned by `EventQueue::push`; any value supplied by the caller is overwritten.
    pub id: EventId,
    pub timestamp: SimTime,
    pub priority: EventPriority,
    /// Assigned by `EventQueue::push`; final tie-breaker that makes ordering total and deterministic.
    pub sequence_no: u64,
    pub source_node: Option<NodeId>,
    pub target_node: Option<NodeId>,
    pub payload: EventPayload,
    /// Informational only. Cancellation is tracked lazily inside `EventQueue`.
    pub cancelled: bool,
}

impl Event {
    /// Builds an unscheduled event. `id` and `sequence_no` are filled in when it is pushed.
    pub fn new(timestamp: SimTime, priority: EventPriority, payload: EventPayload) -> Self {
        Event {
            id: EventId(0),
            timestamp,
            priority,
            sequence_no: 0,
            source_node: None,
            target_node: None,
            payload,
            cancelled: false,
        }
    }

    pub fn with_source(mut self, node: NodeId) -> Self {
        self.source_node = Some(node);
        self
    }

    pub fn with_target(mut self, node: NodeId) -> Self {
        self.target_node = Some(node);
        self
    }

    fn ordering_key(&self) -> (SimTime, EventPriority, u64) {
        (self.timestamp, self.priority, self.sequence_no)
    }
}

/// Two events are equal when they occupy the same position in the schedule.
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.ordering_key() == other.ordering_key()
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Reversed on purpose: `std::collections::BinaryHeap` is a max-heap, so the event with the
/// *smallest* `(timestamp, priority, sequence_no)` must compare as the *greatest*.
impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        other.ordering_key().cmp(&self.ordering_key())
    }
}
