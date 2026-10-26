use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesSample<T> {
    pub timestamp_ns: u64,
    pub value: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesRingBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    len: usize,
}

impl<T: Clone> TimeSeriesRingBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        assert!(capacity > 0, "TimeSeriesRingBuffer capacity must be > 0");
        Self {
            buffer: vec![None; capacity],
            capacity,
            head: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        let write_idx = (self.head + self.len) % self.capacity;
        if self.len < self.capacity {
            self.buffer[write_idx] = Some(item);
            self.len += 1;
        } else {
            // Buffer is full; overwrite oldest at head and advance head
            self.buffer[self.head] = Some(item);
            self.head = (self.head + 1) % self.capacity;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn clear(&mut self) {
        for slot in &mut self.buffer {
            *slot = None;
        }
        self.head = 0;
        self.len = 0;
    }

    pub fn latest(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            let latest_idx = (self.head + self.len - 1) % self.capacity;
            self.buffer[latest_idx].as_ref()
        }
    }

    pub fn oldest(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            self.buffer[self.head].as_ref()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.len).filter_map(move |offset| {
            let idx = (self.head + offset) % self.capacity;
            self.buffer[idx].as_ref()
        })
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_buffer_fifo_overwrite_and_ordering() {
        let mut ring = TimeSeriesRingBuffer::with_capacity(3);
        assert!(ring.is_empty());

        ring.push(10);
        ring.push(20);
        ring.push(30);
        assert!(ring.is_full());
        assert_eq!(ring.to_vec(), vec![10, 20, 30]);

        // Overwrite oldest (10 -> 40, then 20 -> 50)
        ring.push(40);
        assert_eq!(ring.to_vec(), vec![20, 30, 40]);
        assert_eq!(ring.oldest(), Some(&20));
        assert_eq!(ring.latest(), Some(&40));

        ring.push(50);
        assert_eq!(ring.to_vec(), vec![30, 40, 50]);

        ring.clear();
        assert!(ring.is_empty());
        assert_eq!(ring.latest(), None);
    }
}
pub type MetricRingBuffer<T> = TimeSeriesRingBuffer<T>;
