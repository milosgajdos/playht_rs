//! `cargo run --example clone_voices -- "/path/to/voice.m4a" "audio/x-m4a"`
use playht_rs::{
    api::{self, voice::CloneVoiceFileRequest, voice::DeleteClonedVoiceRequest},
    prelude::*,
};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let sample_file = args.next().unwrap();
    let mime_type = args.next().unwrap();

    let req = CloneVoiceFileRequest {
        sample_file,
        mime_type,
        voice_name: "foo-bar".to_owned(),
    };

    let client = api::Client::new();

    let voice = client.clone_voice_from_file(req).await?;
    println!("Got voice clone: {:?}", voice);

    let cloned_voices = client.get_cloned_voices().await?;
    println!("Got voice clones: {:?}", cloned_voices);

    let req = DeleteClonedVoiceRequest { voice_id: voice.id };
    let delete_resp = client.delete_cloned_voice(req).await?;
    println!("Got delete response: {:?}", delete_resp);

    Ok(())
}
