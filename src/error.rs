// TODO: this will be used for
// deserializing errors
#[allow(unused)]
use serde_json::Value;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Client build error: {0}")]
    ClientBuildError(String),
}
