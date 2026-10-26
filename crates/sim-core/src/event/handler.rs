use crate::error::SimResult;
use crate::event::scheduler::EventScheduler;
use crate::event::types::Event;
use crate::telemetry::MetricsSink;
use rand_chacha::ChaCha8Rng;

pub struct EventContext<'a> {
    pub scheduler: &'a mut dyn EventScheduler,
    pub metrics: &'a mut dyn MetricsSink,
    pub rng: &'a mut ChaCha8Rng,
}

pub trait EventHandler {
    fn handle_event(&mut self, event: &Event, ctx: &mut EventContext<'_>) -> SimResult<()>;
}
