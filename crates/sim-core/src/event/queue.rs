use crate::event::types::{Event, EventId};
use crate::time::SimTime;
use std::collections::{BinaryHeap, HashSet};

/// Deterministic min-heap priority queue ordered by `(SimTime, EventPriority, sequence_no)`.
///
/// Cancellation is lazy: cancelled events stay in the heap but are skipped. The queue keeps
/// the invariant that the heap's top is never a cancelled event, so `peek_time` is exact.
#[derive(Debug, Default)]
pub struct EventQueue {
    heap: BinaryHeap<Event>,
    pending_ids: HashSet<EventId>,
    cancelled_ids: HashSet<EventId>,
    next_sequence_no: u64,
}

impl EventQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        EventQueue {
            heap: BinaryHeap::with_capacity(capacity),
            pending_ids: HashSet::with_capacity(capacity),
            cancelled_ids: HashSet::new(),
            next_sequence_no: 0,
        }
    }

    /// Schedules `event`, overwriting its `id` and `sequence_no` with fresh, monotonically
    /// increasing values. Equal-time, equal-priority events therefore pop in push order.
    pub fn push(&mut self, mut event: Event) -> EventId {
        let seq = self.next_sequence_no;
        self.next_sequence_no += 1;
        let id = EventId(seq);
        event.id = id;
        event.sequence_no = seq;
        event.cancelled = false;
        self.pending_ids.insert(id);
        self.heap.push(event);
        id
    }

    /// The id the next pushed event will receive.
    pub fn peek_next_id(&self) -> EventId {
        EventId(self.next_sequence_no)
    }

    pub fn pop_next(&mut self) -> Option<Event> {
        while let Some(event) = self.heap.pop() {
            if self.cancelled_ids.remove(&event.id) {
                continue;
            }
            self.pending_ids.remove(&event.id);
            self.purge_cancelled_top();
            return Some(event);
        }
        None
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.pop_next()
    }

    pub fn peek(&self) -> Option<&Event> {
        self.heap.peek()
    }

    pub fn peek_time(&self) -> Option<SimTime> {
        self.heap.peek().map(|event| event.timestamp)
    }

    /// Returns `false` if the event was never scheduled, already ran, or was already cancelled.
    pub fn cancel(&mut self, event_id: EventId) -> bool {
        if !self.pending_ids.remove(&event_id) {
            return false;
        }
        self.cancelled_ids.insert(event_id);
        self.purge_cancelled_top();
        true
    }

    /// Number of live (scheduled, not cancelled) events.
    pub fn len(&self) -> usize {
        self.pending_ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pending_ids.is_empty()
    }

    /// Drops all events and restarts id allocation at zero.
    pub fn clear(&mut self) {
        self.heap.clear();
        self.pending_ids.clear();
        self.cancelled_ids.clear();
        self.next_sequence_no = 0;
    }

    fn purge_cancelled_top(&mut self) {
        while let Some(top) = self.heap.peek() {
            if !self.cancelled_ids.contains(&top.id) {
                break;
            }
            if let Some(dead) = self.heap.pop() {
                self.cancelled_ids.remove(&dead.id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::types::{EventPayload, EventPriority};

    fn timer(t: u64, priority: EventPriority, timer_id: u64) -> Event {
        Event::new(
            SimTime(t),
            priority,
            EventPayload::TimerFired {
                node: None,
                timer_id,
            },
        )
    }

    fn timer_id(event: &Event) -> u64 {
        match event.payload {
            EventPayload::TimerFired { timer_id, .. } => timer_id,
            _ => unreachable!(),
        }
    }

    #[test]
    fn pops_by_time_then_priority_then_insertion_order() {
        let mut q = EventQueue::new();
        q.push(timer(20, EventPriority::InternalTimer, 1));
        q.push(timer(10, EventPriority::TelemetryFlush, 2));
        q.push(timer(10, EventPriority::ChaosFaultInjection, 3));
        q.push(timer(10, EventPriority::TelemetryFlush, 4));
        q.push(timer(5, EventPriority::TelemetryFlush, 5));

        let order: Vec<u64> = std::iter::from_fn(|| q.pop_next()).map(|e| timer_id(&e)).collect();
        assert_eq!(order, vec![5, 3, 2, 4, 1]);
    }

    #[test]
    fn push_assigns_monotonic_ids() {
        let mut q = EventQueue::new();
        assert_eq!(q.peek_next_id(), EventId(0));
        let a = q.push(timer(1, EventPriority::InternalTimer, 0));
        let b = q.push(timer(1, EventPriority::InternalTimer, 0));
        assert_eq!((a, b), (EventId(0), EventId(1)));
        assert_eq!(q.peek_next_id(), EventId(2));
    }

    #[test]
    fn cancellation_is_exact() {
        let mut q = EventQueue::new();
        let first = q.push(timer(1, EventPriority::InternalTimer, 1));
        let middle = q.push(timer(2, EventPriority::InternalTimer, 2));
        q.push(timer(3, EventPriority::InternalTimer, 3));
        assert_eq!(q.len(), 3);

        assert!(q.cancel(middle));
        assert!(!q.cancel(middle), "double cancel must fail");
        assert!(!q.cancel(EventId(999)), "unknown id must fail");
        assert_eq!(q.len(), 2);

        // Cancelling the top must expose the next live event's time immediately.
        assert!(q.cancel(first));
        assert_eq!(q.peek_time(), Some(SimTime(3)));
        assert_eq!(q.pop_next().map(|e| timer_id(&e)), Some(3));
        assert!(q.is_empty());
        assert_eq!(q.pop_next().map(|e| e.id), None);
    }

    #[test]
    fn cannot_cancel_an_event_that_already_ran() {
        let mut q = EventQueue::new();
        let id = q.push(timer(1, EventPriority::InternalTimer, 1));
        assert!(q.pop_next().is_some());
        assert!(!q.cancel(id));
    }

    #[test]
    fn clear_resets_everything() {
        let mut q = EventQueue::with_capacity(4);
        q.push(timer(1, EventPriority::InternalTimer, 1));
        q.clear();
        assert!(q.is_empty());
        assert_eq!(q.peek_time(), None);
        assert_eq!(q.peek_next_id(), EventId(0));
    }
}
