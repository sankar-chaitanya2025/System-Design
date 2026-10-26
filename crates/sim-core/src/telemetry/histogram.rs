use serde::{Deserialize, Serialize};

const SUB_BUCKET_BITS: u32 = 10;
const SUB_BUCKET_COUNT: usize = 1 << SUB_BUCKET_BITS; // 1024
const SUB_BUCKET_HALF_COUNT: usize = SUB_BUCKET_COUNT >> 1; // 512
const SUB_BUCKET_MASK: usize = SUB_BUCKET_HALF_COUNT - 1; // 511
const MAX_BUCKETS: usize = 56; // Covers full u64 range
const TOTAL_SLOTS: usize = (MAX_BUCKETS + 1) * SUB_BUCKET_HALF_COUNT;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HdrHistogram {
    counts: Vec<u64>,
    total_count: u64,
    min_value: u64,
    max_value: u64,
    sum: u128,
}

impl Default for HdrHistogram {
    fn default() -> Self {
        Self::new()
    }
}

impl HdrHistogram {
    pub fn new() -> Self {
        Self {
            counts: vec![0; TOTAL_SLOTS],
            total_count: 0,
            min_value: u64::MAX,
            max_value: 0,
            sum: 0,
        }
    }

    #[inline]
    fn value_to_index(value: u64) -> usize {
        if value < SUB_BUCKET_COUNT as u64 {
            return value as usize;
        }
        let msb = 63 - value.leading_zeros();
        let bucket = (msb - SUB_BUCKET_BITS + 1) as usize;
        let shift = msb - SUB_BUCKET_BITS + 1;
        let sub_bucket = (value >> shift) as usize;
        ((bucket + 1) << (SUB_BUCKET_BITS - 1)) + (sub_bucket & SUB_BUCKET_MASK)
    }

    #[inline]
    fn index_to_midpoint(index: usize) -> u64 {
        if index < SUB_BUCKET_COUNT {
            return index as u64;
        }
        let bucket = (index >> (SUB_BUCKET_BITS - 1)) - 1;
        let sub_bucket = (index & SUB_BUCKET_MASK) + SUB_BUCKET_HALF_COUNT;
        let shift = bucket as u32;
        let low = (sub_bucket as u64) << shift;
        let high = ((sub_bucket as u64 + 1) << shift) - 1;
        low + ((high - low) >> 1)
    }

    pub fn record(&mut self, value: u64) {
        let idx = Self::value_to_index(value);
        self.counts[idx] += 1;
        self.total_count += 1;
        self.min_value = self.min_value.min(value);
        self.max_value = self.max_value.max(value);
        self.sum += value as u128;
    }

    pub fn percentile(&self, p: f64) -> u64 {
        if self.total_count == 0 {
            return 0;
        }
        let clamped = p.clamp(0.0, 100.0);
        if clamped == 0.0 {
            return self.min_value;
        }
        if clamped >= 100.0 {
            return self.max_value;
        }

        let target_rank = ((clamped / 100.0) * self.total_count as f64).ceil() as u64;
        let target_rank = target_rank.max(1);

        let mut cumulative = 0u64;
        for (idx, &count) in self.counts.iter().enumerate() {
            if count == 0 {
                continue;
            }
            cumulative += count;
            if cumulative >= target_rank {
                return Self::index_to_midpoint(idx);
            }
        }
        self.max_value
    }

    pub fn p50(&self) -> u64 {
        self.percentile(50.0)
    }

    pub fn p90(&self) -> u64 {
        self.percentile(90.0)
    }

    pub fn p95(&self) -> u64 {
        self.percentile(95.0)
    }

    pub fn p99(&self) -> u64 {
        self.percentile(99.0)
    }

    pub fn p999(&self) -> u64 {
        self.percentile(99.9)
    }

    pub fn p9999(&self) -> u64 {
        self.percentile(99.99)
    }

    pub fn mean(&self) -> f64 {
        if self.total_count == 0 {
            0.0
        } else {
            self.sum as f64 / self.total_count as f64
        }
    }

    pub fn count(&self) -> u64 {
        self.total_count
    }

    pub fn min(&self) -> u64 {
        if self.total_count == 0 { 0 } else { self.min_value }
    }

    pub fn max(&self) -> u64 {
        self.max_value
    }

    pub fn reset(&mut self) {
        self.counts.fill(0);
        self.total_count = 0;
        self.min_value = u64::MAX;
        self.max_value = 0;
        self.sum = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    fn oracle_percentile(sorted: &[u64], p: f64) -> u64 {
        let rank = ((p / 100.0) * sorted.len() as f64).ceil() as usize;
        sorted[rank.saturating_sub(1).min(sorted.len() - 1)]
    }

    #[test]
    fn test_hdr_histogram_accuracy_against_sorted_oracle() {
        let mut rng = ChaCha8Rng::seed_from_u64(1337);
        let mut hist = HdrHistogram::new();
        let mut raw = Vec::with_capacity(10_000);

        for _ in 0..10_000 {
            // Span microseconds to seconds in nanoseconds
            let sample: u64 = rng.gen_range(1_000..1_000_000_000);
            hist.record(sample);
            raw.push(sample);
        }

        raw.sort_unstable();

        for &p in &[50.0, 90.0, 95.0, 99.0, 99.9] {
            let expected = oracle_percentile(&raw, p) as f64;
            let actual = hist.percentile(p) as f64;
            let rel_error = (actual - expected).abs() / expected;
            assert!(
                rel_error < 0.01,
                "p{p} relative error {rel_error:.5} exceeded 1% (expected {expected}, got {actual})"
            );
        }
    }
}
pub type LatencyHdrHistogram = HdrHistogram;
