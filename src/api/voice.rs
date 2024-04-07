//! module for cloning and fetching voices from files or URLs.
//!
//! It lets you create, fetch delete cloned voices.

use crate::{api::Client, prelude::*};
use serde::{Deserialize, Serialize};

/// URL path for fetching stock voices.
pub const VOICES_PATH: &str = "/voices";
/// URL path for fetching cloned voices.
pub const CLONED_VOICES_PATH: &str = "/cloned-voices/";
/// URL path for creating cloned voices.
pub const CLONED_VOICES_INSTANT_PATH: &str = "/cloned-voices/instant";

/// Voice metadata
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

/// Cloned voice metadata.
#[derive(Debug, Deserialize, Clone)]
pub struct ClonedVoice {
    pub id: String,
    pub name: String,
    pub r#type: Option<String>,
}

/// Voice cloning request.
#[derive(Debug, Clone)]
pub struct CloneVoiceFileRequest {
    pub sample_file: String,
    pub voice_name: String,
    pub mime_type: String,
}

/// Voice clone URL request.
#[derive(Debug, Serialize, Clone)]
pub struct CloneVoiceURLRequest {
    pub sample_file_url: String,
    pub voice_name: String,
}

/// Voice clone delete request.
#[derive(Debug, Serialize, Clone)]
pub struct DeleteClonedVoiceRequest {
    pub voice_id: String,
}

/// Voice clone success response.
#[derive(Debug, Deserialize, Clone)]
pub struct DeleteClonedVoiceResp {
    pub message: String,
    pub deleted: ClonedVoice,
}

/// Get all available stock voices.
/// Convenience function that does the same thing as [`crate::api::Client::get_stock_voices`].
pub async fn get_stock_voices() -> Result<Vec<Voice>> {
    let voices = Client::new().get_stock_voices().await?;

    Ok(voices)
}

/// Get all cloned voices.
/// Convenience function that does the same thing as [`crate::api::Client::get_cloned_voices`].
pub async fn get_cloned_voices() -> Result<Vec<ClonedVoice>> {
    let voices = Client::new().get_cloned_voices().await?;

    Ok(voices)
}

/// Clone voice from the give file.
/// Convenience function that does the same thing as [`crate::api::Client::clone_voice_from_file`].
pub async fn clone_voice_from_file(req: CloneVoiceFileRequest) -> Result<ClonedVoice> {
    let voice = Client::new().clone_voice_from_file(req).await?;

    Ok(voice)
}

/// Clone voice from the given URL.
/// Convenience function that does the same thing as [`crate::api::Client::clone_voice_from_url`].
pub async fn clone_voice_from_url(req: CloneVoiceURLRequest) -> Result<ClonedVoice> {
    let voice = Client::new().clone_voice_from_url(req).await?;

    Ok(voice)
}

/// Delete cloned voice.
/// Convenience function that does the same thing as [`crate::api::Client::delete_cloned_voice`].
pub async fn delete_cloned_voice(req: DeleteClonedVoiceRequest) -> Result<DeleteClonedVoiceResp> {
    let delete_resp = Client::new().delete_cloned_voice(req).await?;

    Ok(delete_resp)
}
