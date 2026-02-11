// src/protocol/frame.rs

use crate::config::FRAME_SIZE;

#[derive(Clone)]
pub struct AudioFrame {
    pub data: [u8; FRAME_SIZE],
}

impl AudioFrame {
    pub fn new(data: [u8; FRAME_SIZE]) -> Self {
        Self { data }
    }

    pub fn silence() -> Self {
        Self {
            data: [0u8; FRAME_SIZE],
        }
    }
}
