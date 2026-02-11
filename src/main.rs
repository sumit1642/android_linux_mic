// src/main.rs

mod config;
mod protocol;
mod buffer;
mod application;
mod network;
mod monitor;
mod audio;
mod infrastructure;

use application::audio_pipeline::AudioPipeline;
use config::RING_CAPACITY;
use std::io::Read;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

use config::{FRAME_SIZE, PORT};
use protocol::frame::AudioFrame;
fn main() {
    println!("Listening on port {}", PORT);

    // Shared pipeline (network → audio thread)
    let pipeline = Arc::new(Mutex::new(AudioPipeline::new(RING_CAPACITY)));

    // Clone for network thread
    let pipeline_net = Arc::clone(&pipeline);

    // Spawn network thread
    thread::spawn(move || {
        let listener = TcpListener::bind(("0.0.0.0", PORT)).expect("Failed to bind");

        for stream in listener.incoming() {
            let mut stream = stream.expect("Failed to accept connection");
            println!("Client connected: {}", stream.peer_addr().unwrap());

            loop {
                let mut buffer = [0u8; FRAME_SIZE];

                match stream.read_exact(&mut buffer) {
                    Ok(_) => {
                        let frame = AudioFrame::new(buffer);
                        let mut locked = pipeline_net.lock().unwrap();
                        locked.push_frame(frame);
                    }
                    Err(_) => {
                        println!("Client disconnected");
                        break;
                    }
                }
            }
        }
    });

    // Audio thread simulation (main thread)
    loop {
        {
            let mut locked = pipeline.lock().unwrap();

            if let Some(_frame) = locked.pull_frame() {
                println!("Audio thread pulled frame. Buffered: {}", locked.buffered_frames());
            }
        }

        // simulate audio clock ~20ms
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}