use crate::time::SimTime;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TimeSeriesSample<T> {
    pub timestamp: SimTime,
    pub value: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricRingBuffer<T> {
    capacity: usize,
    samples: VecDeque<TimeSeriesSample<T>>,
}

impl<T: Copy> MetricRingBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        todo!()
    }

    pub fn push(&mut self, timestamp: SimTime, value: T) {
        todo!()
    }

    pub fn latest(&self) -> Option<TimeSeriesSample<T>> {
        todo!()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TimeSeriesSample<T>> {
        self.samples.iter()
    }
}
