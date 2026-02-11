// src/config.rs

pub const SAMPLE_RATE: u32 = 48_000;
pub const CHANNELS: u16 = 1;
pub const FRAME_DURATION_MS: u32 = 20;

// 48_000 samples/sec * 0.02 sec = 960 samples
// 960 samples * 2 bytes (16-bit PCM) = 1920 bytes
pub const FRAME_SIZE: usize = 1920;

pub const RING_CAPACITY: usize = 5;

pub const PORT: u16 = 5000;
