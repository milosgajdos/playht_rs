use crate::{
    api::tts::{Emotion, OutputFormat, Quality, VoiceEngine},
    api::Client,
    prelude::*,
};
use serde::{Deserialize, Serialize};

pub const TTS_JOB_PATH: &str = "/tts";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TTSJobReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<Quality>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<OutputFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_engine: Option<VoiceEngine>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotion: Option<Emotion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_guidance: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_guidance: Option<f32>,
}

impl Default for TTSJobReq {
    fn default() -> Self {
        return TTSJobReq {
            text: None,
            voice: None,
            quality: Some(Quality::default()),
            output_format: Some(OutputFormat::default()),
            voice_engine: Some(VoiceEngine::default()),
            emotion: Some(Emotion::default()),
            speed: None,
            temperature: None,
            sample_rate: None,
            seed: None,
            voice_guidance: None,
            style_guidance: None,
        };
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Output {
    pub duration: f64,
    pub size: i32,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Link {
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    pub description: Option<String>,
    pub href: Option<String>,
    pub method: Option<String>,
    pub rel: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TTSJob {
    pub id: String,
    pub created: String,
    pub input: TTSJobReq,
    pub output: Option<Output>,
    // TODO: make status an enum
    pub status: Option<String>,
    #[serde(rename = "_links")]
    pub links: Option<Vec<Link>>,
}

/// Create a new TTS job.
pub async fn create_tts_job(req: TTSJobReq) -> Result<TTSJob> {
    let tts_job = Client::new().create_tts_job(req).await?;

    Ok(tts_job)
}

/// Create a new TTS job and immediately stream its progress.
/// The job progress stream URL is returned if it gets created.
pub async fn create_tts_job_with_progress_stream<W>(
    w: &mut W,
    req: TTSJobReq,
) -> Result<Option<String>>
where
    W: tokio::io::AsyncWriteExt + Unpin,
{
    let stream_url = Client::new()
        .create_tts_job_with_progress_stream(w, req)
        .await?;

    Ok(stream_url)
}

/// Get the TTS job with the given id.
pub async fn get_tts_job(id: String) -> Result<TTSJob> {
    let tts_job = Client::new().get_tts_job(id).await?;

    Ok(tts_job)
}

/// Stream the progress of the TTS job with the given id.
pub async fn stream_tts_job_progress<W>(w: &mut W, id: String) -> Result<()>
where
    W: tokio::io::AsyncWriteExt + Unpin,
{
    Client::new().stream_tts_job_progress(w, id).await?;

    Ok(())
}

/// Stream TTS job audio.
pub async fn stream_tts_job_audio<W>(w: &mut W, id: String) -> Result<()>
where
    W: tokio::io::AsyncWriteExt + Unpin,
{
    Client::new().stream_tts_job_audio(w, id).await?;

    Ok(())
}
