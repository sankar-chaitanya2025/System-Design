use crate::time::{SimDuration, SimTime};
use serde::{Deserialize, Serialize};

/// Models NIC line-rate serialization delay and token-bucket traffic shaping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBucketShaper {
    pub rate_bytes_per_sec: u64,
    pub burst_capacity_bytes: u64,
    pub available_tokens_bytes: f64,
    pub last_refill_time: SimTime,
}

impl TokenBucketShaper {
    pub fn new(rate_bytes_per_sec: u64, burst_capacity_bytes: u64) -> Self {
        todo!()
    }

    pub fn reserve_bandwidth(&mut self, now: SimTime, packet_bytes: u32) -> SimDuration {
        todo!()
    }
}
