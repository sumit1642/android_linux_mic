use crate::protocol::frame::AudioFrame;

pub struct RingBuffer {
    buffer: Vec<Option<AudioFrame>>,
    capacity: usize,
    head: usize, // next write position
    tail: usize, // next read position
    size: usize,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: vec![None; capacity],
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    pub fn push(&mut self, frame: AudioFrame) {
        if self.size == self.capacity {
            // Drop oldest frame
            self.tail = (self.tail + 1) % self.capacity;
            self.size -= 1;
        }

        self.buffer[self.head] = Some(frame);
        self.head = (self.head + 1) % self.capacity;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<AudioFrame> {
        if self.size == 0 {
            return None;
        }

        let frame = self.buffer[self.tail].take();
        self.tail = (self.tail + 1) % self.capacity;
        self.size -= 1;

        frame
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{FRAME_SIZE, RING_CAPACITY};

    fn dummy_frame(byte: u8) -> AudioFrame {
        AudioFrame::new([byte; FRAME_SIZE])
    }

    #[test]
    fn push_and_pop_basic() {
        let mut rb = RingBuffer::new(RING_CAPACITY);

        rb.push(dummy_frame(1));
        assert_eq!(rb.len(), 1);

        let frame = rb.pop().unwrap();
        assert_eq!(frame.data[0], 1);
        assert!(rb.is_empty());
    }

    #[test]
    fn fill_to_capacity() {
        let mut rb = RingBuffer::new(RING_CAPACITY);

        for i in 0..RING_CAPACITY {
            rb.push(dummy_frame(i as u8));
        }

        assert!(rb.is_full());
        assert_eq!(rb.len(), RING_CAPACITY);
    }

    #[test]
    fn overflow_drops_oldest() {
        let mut rb = RingBuffer::new(RING_CAPACITY);

        for i in 0..RING_CAPACITY {
            rb.push(dummy_frame(i as u8));
        }

        // Push one extra (should drop 0)
        rb.push(dummy_frame(99));

        assert_eq!(rb.len(), RING_CAPACITY);

        let first = rb.pop().unwrap();
        assert_eq!(first.data[0], 1); // 0 was dropped
    }

    #[test]
    fn wraparound_behavior() {
        let mut rb = RingBuffer::new(RING_CAPACITY);

        for i in 0..RING_CAPACITY {
            rb.push(dummy_frame(i as u8));
        }

        for _ in 0..2 {
            rb.pop();
        }

        rb.push(dummy_frame(100));
        rb.push(dummy_frame(101));

        assert_eq!(rb.len(), RING_CAPACITY);
    }
}
