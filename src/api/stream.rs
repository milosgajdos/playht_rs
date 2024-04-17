//! module for streaming TTS audio in real-time.
//!
//! You can create new audio streams and stream them in real-time.

use crate::{
    api::tts::{Emotion, OutputFormat, Quality, VoiceEngine},
    api::Client,
    prelude::*,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use tokio_stream::Stream;

/// URL path for fetching the audio streams.
pub const TTS_STREAM_PATH: &str = "/tts/stream";

/// Audio stream request options.
#[derive(Debug, Clone, Serialize)]
#[serde(default)]
pub struct TTSStreamReq {
    pub text: Option<String>,
    pub voice: Option<String>,
    pub quality: Option<Quality>,
    pub output_format: Option<OutputFormat>,
    pub voice_engine: Option<VoiceEngine>,
    pub emotion: Option<Emotion>,
    pub sample_rate: Option<i32>,
    pub seed: Option<i32>,
    pub voice_guidance: Option<f32>,
    pub style_guidance: Option<f32>,
    pub text_guidance: Option<f32>,
    pub temperature: Option<f32>,
    pub speed: Option<f32>,
}

impl Default for TTSStreamReq {
    fn default() -> Self {
        TTSStreamReq {
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
            text_guidance: None,
        }
    }
}

/// Audio stream URL metadata.
#[derive(Debug, Clone, Deserialize)]
pub struct TTSStreamURL {
    pub href: String,
    pub method: String,
    #[serde(rename = "contentType")]
    pub content_type: String,
    pub rel: String,
    pub description: String,
}

/// Writes TTS audio into the given writer.
/// This is a convenience function that does the same thing as [`crate::api::Client::write_audio_stream`].
pub async fn write_audio_stream<W>(w: &mut W, req: &TTSStreamReq) -> Result<()>
where
    W: tokio::io::AsyncWriteExt + Unpin,
{
    Client::new().write_audio_stream(w, req).await?;

    Ok(())
}

/// Fetches the URL for the audio stream.
/// This is a convenience function that does the same thing as [`crate::api::Client::get_audio_stream_url`].
pub async fn get_audio_stream_url(req: &TTSStreamReq) -> Result<TTSStreamURL> {
    let audio_stream_url = Client::new().get_audio_stream_url(req).await?;

    Ok(audio_stream_url)
}

/// Streams raw TTS audio.
/// This is a convenience function that does the same thing as [`crate::api::Client::stream_audio`].
pub async fn stream_audio(req: &TTSStreamReq) -> Result<impl Stream<Item = StreamResult<Bytes>>> {
    let audio_stream = Client::new().stream_audio(req).await?;

    Ok(audio_stream)
}
