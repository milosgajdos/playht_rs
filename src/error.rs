use serde::{self, Deserialize, Deserializer};
use thiserror;

#[derive(Debug, thiserror::Error, Deserialize)]
pub enum Error {
    #[error("Client build error: {0}")]
    ClientBuildError(String),
    #[error("API error")]
    APIError(APIError),
}

#[derive(Debug, thiserror::Error)]
pub enum APIError {
    #[error("Generic API error: {error_message} ({error_id})")]
    Gen {
        error_message: String,
        error_id: String,
    },
    #[error("Internal API error: {message} ({error})")]
    Internal { message: String, error: String },
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
}

impl<'de> Deserialize<'de> for APIError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        if let (Some(error_message), Some(error_id)) = (
            value.get("error_message").and_then(|v| v.as_str()),
            value.get("error_id").and_then(|v| v.as_str()),
        ) {
            return Ok(APIError::Gen {
                error_message: error_message.to_string(),
                error_id: error_id.to_string(),
            });
        }

        if let (Some(message), Some(error)) = (
            value.get("message").and_then(|msg| msg.as_str()),
            value.get("error").and_then(|err| err.as_str()),
        ) {
            return Ok(APIError::Internal {
                message: message.to_string(),
                error: error.to_string(),
            });
        }

        if let Some(rate_limit_error) = value.as_str() {
            return Ok(APIError::RateLimit(rate_limit_error.to_string()));
        }

        Err(serde::de::Error::custom("Unknown API error format"))
    }
}
