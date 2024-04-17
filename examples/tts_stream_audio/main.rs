//! ` cargo run --example tts_stream_audio`
use bytes::BytesMut;
use playht_rs::{
    api::{self, stream::TTSStreamReq, tts::Quality},
    prelude::*,
};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use tokio_stream::StreamExt;

// NOTE: this might need to be adjusted
const BUFFER_SIZE: usize = 1024 * 10;

#[tokio::main]
async fn main() -> Result<()> {
    let client = api::Client::new();
    let voices = client.get_stock_voices().await?;
    if voices.is_empty() {
        return Err("No voices available for playback".into());
    }
    let client = api::Client::new();
    let req = TTSStreamReq {
        text: Some("What is life?".to_owned()),
        voice: Some(voices[0].id.to_owned()),
        quality: Some(Quality::Low),
        speed: Some(1.0),
        sample_rate: Some(24000),
        ..Default::default()
    };

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let mut stream = client.stream_audio(&req).await?;
    let mut accumulated = BytesMut::new();

    while let Some(res) = stream.next().await {
        match res {
            Ok(chunk) => {
                accumulated.extend_from_slice(&chunk);
                // Check if there's enough data to attempt decoding
                if accumulated.len() > BUFFER_SIZE {
                    let cursor = Cursor::new(accumulated.clone().freeze().to_vec());
                    match Decoder::new(cursor) {
                        Ok(source) => {
                            sink.append(source);
                            accumulated.clear(); // Clear the buffer on successful append
                        }
                        Err(e) => {
                            eprintln!("Failed to decode received audio: {}", e);
                        }
                    }
                }
            }
            Err(err) => return Err(format!("Playback error: {}", err).into()),
        }
    }

    // Flush any remaining data at the end
    if !accumulated.is_empty() {
        let cursor = Cursor::new(accumulated.to_vec());
        match Decoder::new(cursor) {
            Ok(source) => sink.append(source),
            Err(e) => println!("Remaining data could not be decoded: {}", e),
        }
    }

    sink.sleep_until_end();
    Ok(())
}
