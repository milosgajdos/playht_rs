use crate::{api::ClientBuilder, error::*, prelude::*};
use bytes::Bytes;
use reqwest::{header::CONTENT_TYPE, Method};
use serde::{Deserialize, Serialize};

pub const PATH: &str = "/voices";

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
    pub r#type: String,
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
pub async fn get_voices() -> Result<Vec<Voice>> {
    let cb = ClientBuilder::new()?;
    let c = cb
        .path(PATH)?
        .header(CONTENT_TYPE.as_str(), APPLICATION_JSON_HEADER)?
        .build()?;
    let req = c.build_request(Method::GET, Bytes::new())?;
    let resp = c.send_request(req).await?;

    if resp.status().is_success() {
        let voices: Vec<Voice> = resp.json().await?;
        Ok(voices)
    } else {
        let api_error: APIError = resp.json().await?;
        Err(Box::new(Error::APIError {
            error_message: api_error.error_message,
            error_id: api_error.error_id,
        }))
    }
}
