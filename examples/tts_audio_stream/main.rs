//! `cargo run --example tts_audio_stream -- "foobar.mp3"`
use playht_rs::{
    api::{self, stream::TTSStreamReq, tts::Quality},
    prelude::*,
};
use tokio::{fs::File, io::BufWriter};

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let file_path = args.next().unwrap();

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

    let file = File::create(file_path.clone()).await?;
    let mut w = BufWriter::new(file);

    api::Client::new().stream_audio(&mut w, req).await?;

    println!("Done streaming into {}", file_path);

    Ok(())
}
