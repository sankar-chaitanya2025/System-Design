use crate::math::LatencyDistributionSpec;
use crate::time::SimDuration;
use crate::topology::LinkCharacteristics;
use rand::Rng;

pub struct NetworkDelayCalculator;

impl NetworkDelayCalculator {
    pub fn compute_one_way_delay<R: Rng>(
        link: &LinkCharacteristics,
        packet_bytes: u32,
        queue_occupancy_ratio: f64,
        rng: &mut R,
    ) -> Option<SimDuration> {
        todo!()
    }
}
