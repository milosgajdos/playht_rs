//! Prelude defines various constants and type aliases.

/// Result type alias used in this crate.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// `application/json` HTTP header
pub const APPLICATION_JSON: &str = "application/json";
/// `multipart/form-data` HTTP header
pub const MULTIPART_FORM: &str = "multipart/form-data";
/// `text/plain` HTTP header
pub const TEXT_PLAIN: &str = "text/plain";
/// `text/event-stream` HTTP header
pub const TEXT_EVENT_STREAM: &str = "text/event-stream";
/// `audio/mpeg` HTTP header
pub const AUDIO_MPEG: &str = "audio/mpeg";
