//! module that defines various TTS data structures.
//!
//! These data structures are used for configuring
//! various properties of TTS streams and jobs.

use serde::{Deserialize, Serialize};

/// play.ht voice engine.
/// It's recommended you use the [`v2`][v2] engine.
///
/// [v2]: VoiceEngine::PlayHTV2
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub enum VoiceEngine {
    #[serde(rename = "PlayHT1.0")]
    PlayHTV1,
    #[serde(rename = "PlayHT2.0")]
    #[default]
    PlayHTV2,
    #[serde(rename = "PlayHT2.0-turbo")]
    PlayHTV2Turbo,
}

/// Supported audio output formats.
/// By default [`mp3`][m] is used.
///
/// [m]: OutputFormat::Mp3
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    #[default]
    Mp3,
    Wav,
    Ogg,
    Flac,
    Mulav,
}

/// Quality of the generated audio stream.
/// By default [`draft`][d] is used.
///
/// [d]: Quality::Draft
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    #[default]
    Draft,
    Low,
    Medium,
    High,
    Premium,
}

/// Emotion in the generated TTS voice.
/// By default [`FemaleHappy`][fh] is used.
///
/// [fh]: Emotion::FemaleHappy
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Emotion {
    #[default]
    FemaleHappy,
    FemaleSad,
    FemaleAngry,
    FemaleFearful,
    FemaleDisgust,
    FemaleSurprised,
    MaleHappy,
    MaleSad,
    MaleAngry,
    MaleFearful,
    MaleDisgust,
    MaleSurprised,
}
