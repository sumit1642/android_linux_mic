mod config;
mod protocol;
mod buffer;
mod application;
mod network;
mod monitor;
mod audio;
mod infrastructure;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use application::audio_pipeline::AudioPipeline;
use config::RING_CAPACITY;
use infrastructure::network::tcp_server::run_server;

fn main() {
    // Shared pipeline (network → audio thread)
    let pipeline = Arc::new(Mutex::new(AudioPipeline::new(RING_CAPACITY)));

    // Spawn network thread
    {
        let pipeline_net = Arc::clone(&pipeline);
        thread::spawn(move || {
            run_server(pipeline_net);
        });
    }

    // Audio thread simulation (main thread)
    loop {
        {
            let mut locked = pipeline.lock().unwrap();

            if let Some(_frame) = locked.pull_frame() {
                println!(
                    "Audio thread pulled frame. Buffered: {}",
                    locked.buffered_frames()
                );
            }
        }

        // Simulated audio clock (~20ms)
        thread::sleep(Duration::from_millis(20));
    }
}
