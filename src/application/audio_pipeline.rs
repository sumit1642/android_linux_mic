// src/application/audio_pipeline.rs
use crate::buffer::ring_buffer::RingBuffer;
use crate::protocol::frame::AudioFrame;

pub struct AudioPipeline {
    buffer: RingBuffer,
}

impl AudioPipeline {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: RingBuffer::new(capacity),
        }
    }

    pub fn push_frame(&mut self, frame: AudioFrame) {
        self.buffer.push(frame);
    }

    pub fn pull_frame(&mut self) -> Option<AudioFrame> {
        self.buffer.pop()
    }

    pub fn buffered_frames(&self) -> usize {
        self.buffer.len()
    }
}
