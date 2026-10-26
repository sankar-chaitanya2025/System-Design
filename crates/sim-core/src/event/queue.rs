use crate::event::types::{Event, EventId};
use crate::time::SimTime;
use std::collections::{BinaryHeap, HashSet};

/// Deterministic min-heap priority queue ordered by `(SimTime, EventPriority, sequence_no)`.
#[derive(Debug, Default)]
pub struct EventQueue {
    heap: BinaryHeap<Event>,
    cancelled_ids: HashSet<EventId>,
    next_sequence_no: u64,
}

impl EventQueue {
    pub fn new() -> Self {
        todo!()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    pub fn push(&mut self, event: Event) -> EventId {
        todo!()
    }

    pub fn pop_next(&mut self) -> Option<Event> {
        todo!()
    }

    pub fn peek_time(&self) -> Option<SimTime> {
        todo!()
    }

    pub fn cancel(&mut self, event_id: EventId) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }
}
