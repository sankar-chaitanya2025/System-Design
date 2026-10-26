use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub};

const NANOS_PER_US: u64 = 1_000;
const NANOS_PER_MS: u64 = 1_000_000;
const NANOS_PER_SEC_F64: f64 = 1_000_000_000.0;
const NANOS_PER_MS_F64: f64 = 1_000_000.0;

/// Nanosecond-precision monotonic virtual simulation timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct SimTime(pub u64);

/// Duration in virtual nanoseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub struct SimDuration(pub u64);

/// Physical node clock drift model (NTP skew + crystal oscillator drift in PPM).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClockSkewModel {
    pub base_offset_ns: i64,
    pub drift_ppm: f64,
    pub max_uncertainty_ns: u64,
}

/// Hybrid Logical Clock (HLC) timestamp for causal ordering across nodes with skewed clocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HybridLogicalClock {
    pub physical_ns: u64,
    pub logical_counter: u32,
    pub node_id: u32,
}

/// Virtual clock controller inside the DES engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualClock {
    current_time: SimTime,
    tick_resolution: SimDuration,
}

impl SimTime {
    pub const ZERO: SimTime = SimTime(0);

    /// Largest timestamp that survives a round-trip through a JavaScript `number`
    /// without precision loss (2^53 - 1 ns, about 104 days of virtual time). See ADR-0001.
    pub const MAX_JS_SAFE: SimTime = SimTime((1u64 << 53) - 1);

    pub fn from_ns(ns: u64) -> Self {
        SimTime(ns)
    }

    pub fn from_us(us: u64) -> Self {
        SimTime(us.saturating_mul(NANOS_PER_US))
    }

    pub fn from_ms(ms: u64) -> Self {
        SimTime(ms.saturating_mul(NANOS_PER_MS))
    }

    /// Negative and NaN inputs map to zero; values beyond `u64::MAX` ns saturate.
    pub fn from_secs_f64(secs: f64) -> Self {
        // `as` casts from f64 to u64 saturate and map NaN to 0.
        SimTime((secs * NANOS_PER_SEC_F64).round() as u64)
    }

    pub fn as_ns(&self) -> u64 {
        self.0
    }

    pub fn as_ms_f64(&self) -> f64 {
        self.0 as f64 / NANOS_PER_MS_F64
    }

    pub fn as_secs_f64(&self) -> f64 {
        self.0 as f64 / NANOS_PER_SEC_F64
    }
}

impl SimDuration {
    pub const ZERO: SimDuration = SimDuration(0);

    pub fn from_ns(ns: u64) -> Self {
        SimDuration(ns)
    }

    pub fn from_us(us: u64) -> Self {
        SimDuration(us.saturating_mul(NANOS_PER_US))
    }

    pub fn from_ms(ms: u64) -> Self {
        SimDuration(ms.saturating_mul(NANOS_PER_MS))
    }

    /// Negative and NaN inputs map to zero; values beyond `u64::MAX` ns saturate.
    pub fn from_secs_f64(secs: f64) -> Self {
        SimDuration((secs * NANOS_PER_SEC_F64).round() as u64)
    }

    pub fn as_ns(&self) -> u64 {
        self.0
    }

    pub fn as_ms_f64(&self) -> f64 {
        self.0 as f64 / NANOS_PER_MS_F64
    }

    pub fn as_secs_f64(&self) -> f64 {
        self.0 as f64 / NANOS_PER_SEC_F64
    }
}

impl Add<SimDuration> for SimTime {
    type Output = SimTime;
    /// Saturates at `u64::MAX` rather than wrapping.
    fn add(self, rhs: SimDuration) -> Self::Output {
        SimTime(self.0.saturating_add(rhs.0))
    }
}

impl AddAssign<SimDuration> for SimTime {
    fn add_assign(&mut self, rhs: SimDuration) {
        *self = *self + rhs;
    }
}

impl Sub<SimTime> for SimTime {
    type Output = SimDuration;
    /// Elapsed time from `rhs` to `self`. Saturates to zero if `rhs` is later than `self`.
    fn sub(self, rhs: SimTime) -> Self::Output {
        SimDuration(self.0.saturating_sub(rhs.0))
    }
}

impl Add<SimDuration> for SimDuration {
    type Output = SimDuration;
    fn add(self, rhs: SimDuration) -> Self::Output {
        SimDuration(self.0.saturating_add(rhs.0))
    }
}

impl VirtualClock {
    /// A zero resolution is coerced to 1 ns.
    pub fn new(resolution: SimDuration) -> Self {
        VirtualClock {
            current_time: SimTime::ZERO,
            tick_resolution: SimDuration(resolution.0.max(1)),
        }
    }

    pub fn now(&self) -> SimTime {
        self.current_time
    }

    pub fn resolution(&self) -> SimDuration {
        self.tick_resolution
    }

    /// Moves the clock forward to `target`. The clock is monotonic: a target in the
    /// past is a caller bug (debug-asserted) and leaves the clock unchanged.
    pub fn advance_to(&mut self, target: SimTime) {
        debug_assert!(
            target >= self.current_time,
            "virtual clock cannot move backwards: {target:?} < {:?}",
            self.current_time
        );
        if target > self.current_time {
            self.current_time = target;
        }
    }

    /// Local wall-clock reading of a node whose oscillator is skewed relative to true
    /// virtual time: `now * (1 + drift_ppm / 1e6) + base_offset_ns`, clamped at zero.
    /// `max_uncertainty_ns` is not applied here (reserved for TrueTime-style intervals).
    pub fn read_skewed(&self, skew: &ClockSkewModel) -> SimTime {
        let now = self.current_time.0 as f64;
        let skewed = now + now * skew.drift_ppm * 1e-6 + skew.base_offset_ns as f64;
        SimTime(skewed.round() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_constructors_convert_to_nanoseconds() {
        assert_eq!(SimTime::from_us(3).as_ns(), 3_000);
        assert_eq!(SimTime::from_ms(2).as_ns(), 2_000_000);
        assert_eq!(SimTime::from_secs_f64(1.5).as_ns(), 1_500_000_000);
        assert_eq!(SimDuration::from_ms(7).as_ns(), 7_000_000);
        assert_eq!(SimTime::from_ms(250).as_ms_f64(), 250.0);
    }

    #[test]
    fn float_constructors_handle_degenerate_inputs() {
        assert_eq!(SimTime::from_secs_f64(-1.0), SimTime::ZERO);
        assert_eq!(SimTime::from_secs_f64(f64::NAN), SimTime::ZERO);
        assert_eq!(SimDuration::from_secs_f64(f64::INFINITY).as_ns(), u64::MAX);
    }

    #[test]
    fn arithmetic_saturates_instead_of_wrapping() {
        assert_eq!(SimTime(u64::MAX) + SimDuration(10), SimTime(u64::MAX));
        assert_eq!(SimTime(5) - SimTime(10), SimDuration::ZERO);
        assert_eq!(SimTime(10) - SimTime(4), SimDuration(6));
        let mut t = SimTime(1);
        t += SimDuration(2);
        assert_eq!(t, SimTime(3));
    }

    #[test]
    fn virtual_clock_is_monotonic() {
        let mut clock = VirtualClock::new(SimDuration::ZERO);
        assert_eq!(clock.resolution(), SimDuration(1));
        clock.advance_to(SimTime(100));
        clock.advance_to(SimTime(100));
        assert_eq!(clock.now(), SimTime(100));
    }

    #[test]
    fn skewed_reading_applies_drift_and_offset() {
        let mut clock = VirtualClock::new(SimDuration(1));
        clock.advance_to(SimTime::from_ms(1_000));
        let skew = ClockSkewModel {
            base_offset_ns: -500,
            drift_ppm: 100.0,
            max_uncertainty_ns: 0,
        };
        // 1e9 ns * 100 ppm = 100_000 ns of drift, minus a 500 ns offset.
        assert_eq!(clock.read_skewed(&skew), SimTime(1_000_000_000 + 100_000 - 500));
    }
}
