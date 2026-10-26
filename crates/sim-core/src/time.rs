use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

/// Picosecond-precision or Nanosecond-precision monotonic virtual simulation timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct SimTime(pub u64);

/// Duration in virtual nanoseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct SimDuration(pub u64);

/// Physical node clock drift model (NTP skew + crystal oscillator drift in PPM).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ClockSkewModel {
    pub base_offset_ns: i64,
    pub drift_ppm: f64,
    pub max_uncertainty_ns: u64,
}

/// Hybrid Logical Clock (HLC) timestamp for causal ordering across nodes with skewed clocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HybridLogicalClock {
    pub physical_ns: u64,
    pub logical_counter: u32,
    pub node_id: u32,
}

/// Virtual clock controller inside the DES engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualClock {
    current_time: SimTime,
    tick_resolution: SimDuration,
}

impl SimTime {
    pub const ZERO: SimTime = SimTime(0);

    pub fn from_ns(ns: u64) -> Self {
        todo!()
    }

    pub fn from_us(us: u64) -> Self {
        todo!()
    }

    pub fn from_ms(ms: u64) -> Self {
        todo!()
    }

    pub fn from_secs_f64(secs: f64) -> Self {
        todo!()
    }

    pub fn as_ns(&self) -> u64 {
        todo!()
    }

    pub fn as_ms_f64(&self) -> f64 {
        todo!()
    }
}

impl SimDuration {
    pub fn from_ns(ns: u64) -> Self {
        todo!()
    }

    pub fn from_us(us: u64) -> Self {
        todo!()
    }

    pub fn from_ms(ms: u64) -> Self {
        todo!()
    }

    pub fn as_ns(&self) -> u64 {
        todo!()
    }
}

impl Add<SimDuration> for SimTime {
    type Output = SimTime;
    fn add(self, _rhs: SimDuration) -> Self::Output {
        todo!()
    }
}

impl Sub<SimTime> for SimTime {
    type Output = SimDuration;
    fn sub(self, _rhs: SimTime) -> Self::Output {
        todo!()
    }
}

impl VirtualClock {
    pub fn new(resolution: SimDuration) -> Self {
        todo!()
    }

    pub fn now(&self) -> SimTime {
        todo!()
    }

    pub fn advance_to(&mut self, target: SimTime) {
        todo!()
    }

    pub fn read_skewed(&self, skew: &ClockSkewModel) -> SimTime {
        todo!()
    }
}
