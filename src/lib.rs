//! play.ht RS
//!
//! An unofficial play.ht API client library.
//!
//! play.ht homesite: <https://play.ht/>
//!
//! play.ht API docs: <https://docs.play.ht/reference/api-getting-started>
//!

pub use crate::api::{
    job::{
        create_tts_job, create_tts_job_write_progress_stream, get_tts_job, stream_tts_job_progress,
        write_tts_job_audio_stream, write_tts_job_progress_stream,
    },
    stream::{get_audio_stream_url, stream_audio, write_audio_stream},
    voice::{
        clone_voice_from_file, clone_voice_from_url, delete_cloned_voice, get_cloned_voices,
        get_stock_voices,
    },
};

pub mod api;
pub mod error;
pub mod prelude;
