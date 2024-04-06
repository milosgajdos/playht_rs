use playht_rs::{
    api::{self, voice::CloneVoiceFileRequest},
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

    let voice = api::Client::new().clone_voice_from_file(req).await?;

    println!("Got {:?} voices", voice);

    Ok(())
}
