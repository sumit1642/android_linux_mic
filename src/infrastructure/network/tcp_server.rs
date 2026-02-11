use std::io::Read;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use crate::application::audio_pipeline::AudioPipeline;
use crate::config::{FRAME_SIZE, PORT};
use crate::protocol::frame::AudioFrame;

pub fn run_server(pipeline: Arc<Mutex<AudioPipeline>>) {
    let listener =
        TcpListener::bind(("0.0.0.0", PORT)).expect("Failed to bind TCP server");

    println!("Listening on port {}", PORT);

    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept connection");
        println!("Client connected: {}", stream.peer_addr().unwrap());

        loop {
            let mut buffer = [0u8; FRAME_SIZE];

            match stream.read_exact(&mut buffer) {
                Ok(_) => {
                    let frame = AudioFrame::new(buffer);
                    let mut locked = pipeline.lock().unwrap();
                    locked.push_frame(frame);
                }
                Err(_) => {
                    println!("Client disconnected");
                    break;
                }
            }
        }
    }
}
