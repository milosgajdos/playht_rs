pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const APPLICATION_JSON: &str = "application/json";
pub const MULTIPART_FORM: &str = "multipart/form-data";
pub const TEXT_PLAIN: &str = "text/plain";
pub const TEXT_EVENT_STREAM: &str = "text/event-stream";
