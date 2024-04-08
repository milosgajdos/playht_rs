//! `cargo run --example play_tts_audio_stream`
use playht_rs::{
    api::{self, stream::TTSStreamReq, tts::Quality},
    prelude::*,
};
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    let client = api::Client::new();
    let voices = client.get_stock_voices().await?;
    if voices.is_empty() {
        return Err("No voices available".into());
    }

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

    let mut buffer = Vec::new();
    client.stream_audio(&mut buffer, req).await?;

    let source = Decoder::new(Cursor::new(buffer)).unwrap();
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
