//! `cargo run --example tts_jobs`
use playht_rs::{
    api::{self, job::TTSJobReq, tts::Quality},
    prelude::*,
};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let client = api::Client::new();
    let voices = client.get_stock_voices().await?;
    if voices.is_empty() {
        return Err("No voices available".into());
    }

    let req = TTSJobReq {
        text: Some("What is life?".to_owned()),
        voice: Some(voices[0].id.clone()),
        quality: Some(Quality::Low),
        speed: Some(1.0),
        sample_rate: Some(24000),
        ..Default::default()
    };

    let tts_job = client.create_tts_job(&req).await?;
    println!("TTS job created: {:?}", tts_job);

    let tts_job = client.get_tts_job(tts_job.id).await?;
    println!("Got TTS job: {:?}", tts_job);

    Ok(())
}
