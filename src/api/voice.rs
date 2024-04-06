use crate::{api::Client, prelude::*};
use serde::{Deserialize, Serialize};

pub const VOICES_PATH: &str = "/voices";
pub const CLONED_VOICES_PATH: &str = "/cloned-voices";
pub const CLONE_VOICE_PATH: &str = "/cloned-voices/instant";

#[derive(Debug, Deserialize, Clone)]
pub struct Voice {
    pub id: String,
    pub name: String,
    pub sample: Option<String>,
    pub accent: Option<String>,
    pub age: Option<String>,
    pub gender: Option<String>,
    pub language: Option<String>,
    pub lang_code: Option<String>,
    pub loudness: Option<String>,
    pub style: Option<String>,
    pub tempo: Option<String>,
    pub texture: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClonedVoice {
    pub id: String,
    pub name: String,
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CloneVoiceFileRequest {
    pub sample_file: String,
    pub voice_name: String,
    pub mime_type: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct CloneVoiceURLRequest {
    pub sample_file_url: String,
    pub voice_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct DeleteClonedVoiceRequest {
    pub voice_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteClonedVoiceResp {
    pub message: String,
    pub deleted: ClonedVoice,
}

/// Get all available stock Voices.
pub async fn get_stock_voices() -> Result<Vec<Voice>> {
    let voices = Client::new().get_stock_voices().await?;

    Ok(voices)
}

/// Get all cloned Voices.
pub async fn get_cloned_voices() -> Result<Vec<ClonedVoice>> {
    let voices = Client::new().get_cloned_voices().await?;

    Ok(voices)
}

/// Clone voice from the file specified via req.
pub async fn clone_voice_from_file(req: CloneVoiceFileRequest) -> Result<ClonedVoice> {
    let voice = Client::new().clone_voice_from_file(req).await?;

    Ok(voice)
}
