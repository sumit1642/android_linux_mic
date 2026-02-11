// src/infrastructure/network/tcp_server.rs

use std::io::Read;
use std::net::{TcpListener, TcpStream};

use crate::application::audio_pipeline::AudioPipeline;
use crate::config::{FRAME_SIZE, PORT};
use crate::protocol::frame::AudioFrame;

pub fn run_server(pipeline: &mut AudioPipeline) -> std::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", PORT))?;
    println!("Listening on port {}", PORT);

    let (mut stream, addr) = listener.accept()?;
    println!("Client connected: {}", addr);

    handle_client(&mut stream, pipeline)
}

fn handle_client(
    stream: &mut TcpStream,
    pipeline: &mut AudioPipeline,
) -> std::io::Result<()> {
    loop {
        let mut buffer = [0u8; FRAME_SIZE];

        stream.read_exact(&mut buffer)?;

        let frame = AudioFrame::new(buffer);
        pipeline.push_frame(frame);

        println!("Buffered frames: {}", pipeline.buffered_frames());
    }
}
