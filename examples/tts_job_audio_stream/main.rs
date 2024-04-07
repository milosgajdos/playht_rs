//! `cargo run --example tts_job_audio_stream -- "job-id" "/path/to/output.mp3"`
use playht_rs::{api, api::job, prelude::*};
use tokio::{fs::File, io::BufWriter};

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let job_id = args.next().unwrap();
    let file_path = args.next().unwrap();

    let tts_job = job::get_tts_job(job_id.clone()).await?;
    println!("Got TTS job: {}", tts_job.id);

    // TODO: we should make status an enum
    if let Some(status) = tts_job.status {
        if status == "failed" {
            println!("Cant stream: {} has failed", tts_job.id);
            return Ok(());
        }
    }

    let file = File::create(file_path.clone()).await?;
    let mut w = BufWriter::new(file);
    api::Client::new()
        .stream_tts_job_audio(&mut w, job_id)
        .await?;

    println!("Done streaming into {}", file_path);

    Ok(())
}
