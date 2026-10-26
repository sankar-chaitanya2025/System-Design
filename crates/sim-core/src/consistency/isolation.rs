use serde::{Deserialize, Serialize};

/// DDIA Chapter 7 Transaction Isolation Levels & Concurrency Control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IsolationLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableReadSnapshotIsolation,
    SerializableTwoPhaseLocking,
    SerializableSnapshotIsolationSsi,
}

impl IsolationLevel {
    pub fn throughput_penalty_multiplier(&self, write_contention_ratio: f64) -> f64 {
        todo!()
    }

    pub fn abort_retry_probability(&self, hot_key_skew: f64) -> f64 {
        todo!()
    }
}
