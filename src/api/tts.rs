use serde::{Deserialize, Serialize};

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
