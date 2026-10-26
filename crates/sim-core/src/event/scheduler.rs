use crate::error::{SimError, SimResult};
use crate::event::queue::EventQueue;
use crate::event::types::{Event, EventId, EventPayload, EventPriority};
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

/// `EventScheduler` over the engine's queue, frozen at the timestamp of the event being
/// handled. Scheduling into the past is rejected with `SimError::CausalityViolation`.
pub struct QueueScheduler<'q> {
    queue: &'q mut EventQueue,
    now: SimTime,
}

impl<'q> QueueScheduler<'q> {
    pub fn new(queue: &'q mut EventQueue, now: SimTime) -> Self {
        QueueScheduler { queue, now }
    }
}

impl EventScheduler for QueueScheduler<'_> {
    fn now(&self) -> SimTime {
        self.now
    }

    fn schedule_at(
        &mut self,
        timestamp: SimTime,
        priority: EventPriority,
        source: Option<NodeId>,
        target: Option<NodeId>,
        payload: EventPayload,
    ) -> SimResult<EventId> {
        if timestamp < self.now {
            return Err(SimError::CausalityViolation {
                event_id: self.queue.peek_next_id(),
                scheduled_at: timestamp,
                current_clock: self.now,
            });
        }
        let mut event = Event::new(timestamp, priority, payload);
        event.source_node = source;
        event.target_node = target;
        Ok(self.queue.push(event))
    }

    fn schedule_after(
        &mut self,
        delay: SimDuration,
        priority: EventPriority,
        source: Option<NodeId>,
        target: Option<NodeId>,
        payload: EventPayload,
    ) -> SimResult<EventId> {
        self.schedule_at(self.now + delay, priority, source, target, payload)
    }

    fn cancel_event(&mut self, event_id: EventId) -> bool {
        self.queue.cancel(event_id)
    }
}
