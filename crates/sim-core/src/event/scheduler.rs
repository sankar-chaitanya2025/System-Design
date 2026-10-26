use crate::error::SimResult;
use crate::event::types::{EventId, EventPayload, EventPriority};
use crate::time::{SimDuration, SimTime};
use crate::topology::NodeId;

/// Scoped event emitter passed into node handlers during simulation execution.
pub trait EventScheduler {
    fn now(&self) -> SimTime;

    fn schedule_at(
        &mut self,
        timestamp: SimTime,
        priority: EventPriority,
        source: Option<NodeId>,
        target: Option<NodeId>,
        payload: EventPayload,
    ) -> SimResult<EventId>;

    fn schedule_after(
        &mut self,
        delay: SimDuration,
        priority: EventPriority,
        source: Option<NodeId>,
        target: Option<NodeId>,
        payload: EventPayload,
    ) -> SimResult<EventId>;

    fn cancel_event(&mut self, event_id: EventId) -> bool;
}
