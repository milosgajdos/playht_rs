//! Play.ht RS
//!
//! An unofficial Play.ht API client library.
//!
//! Play.ht homesite: https://play.ht/
//!
//! Play.ht API docs: https://docs.play.ht/reference/api-getting-started
//!

pub use crate::api::{
    job::{
        create_tts_job, create_tts_job_with_progress_stream, get_tts_job, stream_tts_job_audio,
        stream_tts_job_progress,
    },
    voice::{
        clone_voice_from_file, clone_voice_from_url, delete_cloned_voice, get_cloned_voices,
        get_stock_voices,
    },
};

pub mod api;
pub mod error;
pub mod prelude;
