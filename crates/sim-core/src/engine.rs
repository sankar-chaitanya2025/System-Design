use crate::chaos::{ChaosFaultId, ChaosSchedule, FaultEngine};
use crate::config::SimulationConfig;
use crate::error::{SimError, SimResult};
use crate::event::{
    Event, EventId, EventPayload, EventPriority, EventQueue, EventScheduler, QueueScheduler,
};
use crate::payloads::FluidDensityWave;
use crate::serialization::EngineSnapshot;
use crate::telemetry::TelemetryCollector;
use crate::time::{SimDuration, SimTime, VirtualClock};
use crate::topology::{EdgeId, NodeId, TopologyGraph};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EngineRunState {
    Idle,
    Running,
    Paused,
    Stepping,
    Completed,
    FaultHalted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StepExecutionSummary {
    pub start_time: SimTime,
    pub end_time: SimTime,
    pub discrete_events_processed: usize,
    pub fluid_steps_integrated: usize,
    pub active_packets_in_flight: usize,
    pub dropped_requests_delta: u64,
}

/// Deterministic record of a processed event used for replay verification and debugging.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutedEventRecord {
    pub event_id: EventId,
    pub timestamp: SimTime,
    pub priority: EventPriority,
    pub target_node: Option<NodeId>,
    pub payload_tag: &'static str,
}

pub struct SimulationEngine {
    config: SimulationConfig,
    run_state: EngineRunState,
    clock: VirtualClock,
    event_queue: EventQueue,
    topology: Option<TopologyGraph>,
    telemetry: Option<TelemetryCollector>,
    fault_engine: Option<FaultEngine>,
    rng: ChaCha8Rng,
    execution_trace: Vec<ExecutedEventRecord>,
}

impl SimulationEngine {
    pub fn new(config: SimulationConfig) -> Self {
        let rng = ChaCha8Rng::seed_from_u64(config.random_seed);
        Self {
            config,
            run_state: EngineRunState::Idle,
            clock: VirtualClock::new(SimDuration::from_ns(1)),
            event_queue: EventQueue::new(),
            topology: None,
            telemetry: None,
            fault_engine: None,
            rng,
            execution_trace: Vec::new(),
        }
    }

    pub fn from_snapshot(_snapshot: EngineSnapshot) -> SimResult<Self> {
        Err(SimError::Unsupported(
            "Snapshots are deferred to Phase 10 (ADR-0006)".into(),
        ))
    }

    pub fn create_snapshot(&self) -> SimResult<EngineSnapshot> {
        Err(SimError::Unsupported(
            "Snapshots are deferred to Phase 10 (ADR-0006)".into(),
        ))
    }

    pub fn reset(&mut self, seed: Option<u64>) {
        if let Some(new_seed) = seed {
            self.config.random_seed = new_seed;
        }
        self.run_state = EngineRunState::Idle;
        self.clock = VirtualClock::new(self.clock.resolution());
        self.event_queue.clear();
        self.rng = ChaCha8Rng::seed_from_u64(self.config.random_seed);
        self.execution_trace.clear();
    }

    pub fn step_single_event(&mut self) -> SimResult<Option<Event>> {
        self.ensure_not_halted()?;

        let Some(event) = self.event_queue.pop() else {
            self.run_state = EngineRunState::Completed;
            return Ok(None);
        };

        self.run_state = EngineRunState::Stepping;
        self.clock.advance_to(event.timestamp);
        self.dispatch_event(&event)?;

        if self.event_queue.is_empty() {
            self.run_state = EngineRunState::Completed;
        }

        Ok(Some(event))
    }

    pub fn advance_by(&mut self, delta: SimDuration) -> SimResult<StepExecutionSummary> {
        let target_time = self.clock.now() + delta;
        self.advance_until(target_time)
    }

    pub fn advance_until(&mut self, target_time: SimTime) -> SimResult<StepExecutionSummary> {
        self.ensure_not_halted()?;

        let start_time = self.clock.now();
        if target_time < start_time {
            return Err(SimError::InvalidTimeTarget {
                target: target_time,
                current: start_time,
            });
        }

        self.run_state = EngineRunState::Running;
        let max_budget = self.config.max_events_per_frame;
        let mut discrete_events_processed = 0usize;
        let mut hit_frame_budget = false;

        while let Some(next_event) = self.event_queue.peek() {
            if next_event.timestamp > target_time {
                break;
            }
            if discrete_events_processed >= max_budget {
                hit_frame_budget = true;
                break;
            }

            let event = self
                .event_queue
                .pop()
                .expect("peeked event must exist in queue");
            self.clock.advance_to(event.timestamp);
            self.dispatch_event(&event)?;
            discrete_events_processed += 1;
        }

        // Only advance the clock to `target_time` if we did not yield early on the frame budget.
        // If we hit `max_events_per_frame`, earlier events <= target_time may still sit in the queue;
        // jumping the clock past them would trigger a causality violation on the next frame.
        if !hit_frame_budget {
            self.clock.advance_to(target_time);
        }

        if self.event_queue.is_empty() {
            self.run_state = EngineRunState::Completed;
        }

        Ok(StepExecutionSummary {
            start_time,
            end_time: self.clock.now(),
            discrete_events_processed,
            fluid_steps_integrated: 0,
            active_packets_in_flight: 0,
            dropped_requests_delta: 0,
        })
    }

    pub fn schedule_event(&mut self, event: Event) -> SimResult<EventId> {
        self.ensure_not_halted()?;
        let mut scheduler = QueueScheduler::new(&mut self.event_queue, self.clock.now());
        scheduler.schedule_at(
            event.timestamp,
            event.priority,
            event.source_node,
            event.target_node,
            event.payload,
        )
    }

    pub fn cancel_event(&mut self, event_id: EventId) -> bool {
        let mut scheduler = QueueScheduler::new(&mut self.event_queue, self.clock.now());
        scheduler.cancel_event(event_id)
    }

    pub fn integrate_fluid_step(&mut self, _dt: SimDuration) -> SimResult<()> {
        Err(SimError::Unsupported(
            "Macro-fluid integration is deferred to Phase 9 (ADR-0006)".into(),
        ))
    }

    pub fn inject_chaos_fault(&mut self, _schedule: ChaosSchedule) -> SimResult<ChaosFaultId> {
        Err(SimError::Unsupported(
            "Chaos fault injection is deferred to Phase 6 (ADR-0006)".into(),
        ))
    }

    pub fn remove_chaos_fault(&mut self, _fault_id: ChaosFaultId) -> SimResult<()> {
        Err(SimError::Unsupported(
            "Chaos fault removal is deferred to Phase 6 (ADR-0006)".into(),
        ))
    }

    pub fn topology(&self) -> &TopologyGraph {
        self.topology
            .as_ref()
            .expect("TopologyGraph is initialized in Phase 2")
    }

    pub fn topology_mut(&mut self) -> &mut TopologyGraph {
        self.topology
            .as_mut()
            .expect("TopologyGraph is initialized in Phase 2")
    }

    pub fn telemetry(&self) -> &TelemetryCollector {
        self.telemetry
            .as_ref()
            .expect("TelemetryCollector is initialized in Phase 2")
    }

    pub fn telemetry_mut(&mut self) -> &mut TelemetryCollector {
        self.telemetry
            .as_mut()
            .expect("TelemetryCollector is initialized in Phase 2")
    }

    pub fn current_time(&self) -> SimTime {
        self.clock.now()
    }

    pub fn pending_event_count(&self) -> usize {
        self.event_queue.len()
    }

    pub fn run_state(&self) -> EngineRunState {
        self.run_state
    }

    pub fn rng_mut(&mut self) -> &mut ChaCha8Rng {
        &mut self.rng
    }

    pub fn execution_trace(&self) -> &[ExecutedEventRecord] {
        &self.execution_trace
    }

    fn ensure_not_halted(&self) -> SimResult<()> {
        if self.run_state == EngineRunState::FaultHalted {
            return Err(SimError::EngineHalted);
        }
        Ok(())
    }

    /// Phase 1 echo/stub event handler: records the processed event into the deterministic trace.
    fn dispatch_event(&mut self, event: &Event) -> SimResult<()> {
        let payload_tag = match &event.payload {
            EventPayload::TimerFired { .. } => "timerFired",
            _ => "other",
        };

        self.execution_trace.push(ExecutedEventRecord {
            event_id: event.id,
            timestamp: event.timestamp,
            priority: event.priority,
            target_node: event.target_node,
            payload_tag,
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn make_timer_event(timestamp: SimTime, priority: EventPriority, tag: u64) -> Event {
        Event::new(
            timestamp,
            priority,
            EventPayload::TimerFired {
                node: None,
                timer_id: tag,
            },
        )
    }

    #[test]
    fn determinism_property_same_seed_produces_byte_identical_traces() {
        fn run_simulation(seed: u64) -> Vec<u8> {
            let mut config = SimulationConfig::default();
            config.random_seed = seed;
            config.max_events_per_frame = 10_000;

            let mut engine = SimulationEngine::new(config);

            let priorities = [
                EventPriority::ConsensusControl,
                EventPriority::NetworkArrival,
                EventPriority::InternalTimer,
                EventPriority::RequestIngress,
                EventPriority::TelemetryFlush,
            ];

            // Schedule 500 pseudo-random events with deliberate timestamp ties and varied priorities
            for i in 0..500u64 {
                let ts_ns = engine.rng_mut().gen_range(1..=250) * 1_000;
                let prio_idx = engine.rng_mut().gen_range(0..priorities.len());
                let event = make_timer_event(SimTime::from_ns(ts_ns), priorities[prio_idx], i);
                engine.schedule_event(event).unwrap();
            }

            engine
                .advance_until(SimTime::from_ns(500_000))
                .expect("simulation run should succeed");

            assert_eq!(engine.pending_event_count(), 0);
            assert_eq!(engine.run_state(), EngineRunState::Completed);

            serde_json::to_vec(engine.execution_trace()).unwrap()
        }

        let trace_a = run_simulation(0xDEAD_BEEF_CAFE_BABEu64);
        let trace_b = run_simulation(0xDEAD_BEEF_CAFE_BABEu64);
        let trace_different = run_simulation(0x1234_5678_9ABC_DEF0u64);

        assert_eq!(
            trace_a, trace_b,
            "Identical seeds must produce 100% byte-identical execution traces"
        );
        assert_ne!(
            trace_a, trace_different,
            "Different seeds must diverge in their execution traces"
        );
    }

    #[test]
    fn frame_budget_guard_yields_early_without_breaking_causality() {
        let mut config = SimulationConfig::default();
        config.max_events_per_frame = 3;

        let mut engine = SimulationEngine::new(config);
        for t in 1..=10 {
            engine
                .schedule_event(make_timer_event(
                    SimTime::from_ns(t * 100),
                    EventPriority::InternalTimer,
                    t,
                ))
                .unwrap();
        }

        // First frame should process only 3 events and stop at t = 300ns
        let summary_1 = engine.advance_until(SimTime::from_ns(1_000)).unwrap();
        assert_eq!(summary_1.discrete_events_processed, 3);
        assert_eq!(engine.current_time(), SimTime::from_ns(300));
        assert_eq!(engine.pending_event_count(), 7);

        // Drain the rest in subsequent frames
        let summary_2 = engine.advance_until(SimTime::from_ns(1_000)).unwrap();
        assert_eq!(summary_2.discrete_events_processed, 3);
        assert_eq!(engine.current_time(), SimTime::from_ns(600));

        let summary_3 = engine.advance_until(SimTime::from_ns(1_000)).unwrap();
        assert_eq!(summary_3.discrete_events_processed, 3);
        assert_eq!(engine.current_time(), SimTime::from_ns(900));

        let summary_4 = engine.advance_until(SimTime::from_ns(1_000)).unwrap();
        assert_eq!(summary_4.discrete_events_processed, 1);
        assert_eq!(engine.current_time(), SimTime::from_ns(1_000));
        assert_eq!(engine.run_state(), EngineRunState::Completed);
    }

    #[test]
    fn rejects_scheduling_in_the_past() {
        let mut engine = SimulationEngine::new(SimulationConfig::default());
        engine.advance_by(SimDuration::from_ns(1_000)).unwrap();

        let past_event = make_timer_event(
            SimTime::from_ns(500),
            EventPriority::InternalTimer,
            1,
        );
        assert!(engine.schedule_event(past_event).is_err());
    }
}