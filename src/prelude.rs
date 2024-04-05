pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const APPLICATION_JSON_HEADER: &str = "application/json";
