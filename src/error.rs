use serde::{self, Deserialize};
use thiserror;

#[derive(Debug, thiserror::Error, Deserialize)]
pub enum Error {
    #[error("Client build error: {0}")]
    ClientBuildError(String),
    #[error("API error: {error_message} ({error_id})")]
    APIError {
        error_message: String,
        error_id: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct APIError {
    pub error_message: String,
    pub error_id: String,
}
