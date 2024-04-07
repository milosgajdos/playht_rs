# playht_rs

[![Build Status](https://github.com/milosgajdos/playht_rs/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/milosgajdos/playht_rs/actions?query=workflow%3ACI)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

An unofficial [play.ht](https://play.ht) Rust API client crate. Similar to the [Go module](https://github.com/milosgajdos/go-playht) implementation.

In order to use this create you must create an account on [play.ht](https://play.ht), generate an API secret and retrieve your User ID.
See the official docs [here](https://docs.play.ht/reference/api-authentication) for more info.

# Basics

There are two ways to create audio/speech from the text using the API:

- Job: audio generation is done in async; when you create a job you can monitor its progress via [SSE](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events)
- Stream: a real-time audio stream available immediately as soon as the stream has been created via the API

The API also allows you to clone a voice using a small sample of limited size. See the [docs](https://docs.play.ht/reference/api-create-instant-voice-clone).

# Get started

> [!IMPORTANT]
> Before you attempt to run any of the samples you must set a couple of environment variables.
> These are automatically read by the client when it gets created; you can override them in your own code.

- `PLAYHT_SECRET_KEY`: API secret key
- `PLAYHT_USER_ID`: Play.HT User ID

Check the crate:

```
cargo check
```

Build the crate:

```shell
cargo build
```

## Examples

There are quite a few code samples available in the [examples](./examples) directory so please do have a look. They could give you some idea about how to use this crate. Below we list a few code samples:

### Clone Voice

This example demonstrates how you can clone a new voice from a sample audio file.

> [!NOTE]
> You must pass the sample file and the mime type as cli arguments

```rust
//! `cargo run --example get_voices`
use playht_rs::{
    api::{self, voice::CloneVoiceFileRequest, voice::DeleteClonedVoiceRequest},
    prelude::*,
};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let sample_file = args.next().unwrap();
    let mime_type = args.next().unwrap();

    let req = CloneVoiceFileRequest {
        sample_file,
        mime_type,
        voice_name: "foo-bar".to_owned(),
    };

    let client = api::Client::new();

    let voice = client.clone_voice_from_file(req).await?;
    println!("Got voice clone: {:?}", voice);

    let cloned_voices = client.get_cloned_voices().await?;
    println!("Got voice clones: {:?}", cloned_voices);

    let req = DeleteClonedVoiceRequest { voice_id: voice.id };
    let delete_resp = client.delete_cloned_voice(req).await?;
    println!("Got delete response: {:?}", delete_resp);

    Ok(())
}
```

### Creat async TTS Jobs

This example demonstrates how you can create an async TTS job.

> [!NOTE]
> The async TTS job progress can be monitored via the API.

```rust
//! `cargo run --example tts_jobs`
use playht_rs::{
    api::{self, job::TTSJobReq, tts::Quality},
    prelude::*,
};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let client = api::Client::new();
    let voices = client.get_stock_voices().await?;
    if voices.is_empty() {
        return Err("No voices available".into());
    }

    let req = TTSJobReq {
        text: Some("What is life?".to_owned()),
        voice: Some(voices[0].id.clone()),
        quality: Some(Quality::Low),
        speed: Some(1.0),
        sample_rate: Some(24000),
        ..Default::default()
    };

    let tts_job = client.create_tts_job(req).await?;
    println!("TTS job created: {:?}", tts_job);

    let tts_job = client.get_tts_job(tts_job.id).await?;
    println!("Got TTS job: {:?}", tts_job);

    Ok(())
}
```

### Stream TTS Audio in real-time

This example demonstrates how you can stream the TTS audio in real-time.
In this case we are storing it in a file specified via a cli argument but you can pass in any async writer implementation such as an audio device tokio wrapper, etc.

> [!NOTE]
> You must pass the output file path as cli argument.

```rust
//! `cargo run --example tts_audio_stream -- "foobar.mp3"`
use playht_rs::{
    api::{self, stream::TTSStreamReq, tts::Quality},
    prelude::*,
};
use tokio::{fs::File, io::BufWriter};

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let file_path = args.next().unwrap();

    let client = api::Client::new();
    let voices = client.get_stock_voices().await?;
    if voices.is_empty() {
        return Err("No voices available".into());
    }

    let req = TTSStreamReq {
        text: Some("What is life?".to_owned()),
        voice: Some(voices[0].id.to_owned()),
        quality: Some(Quality::Low),
        speed: Some(1.0),
        sample_rate: Some(24000),
        ..Default::default()
    };
    let file = File::create(file_path.clone()).await?;
    let mut w = BufWriter::new(file);
    client.stream_audio(&mut w, req).await?;
    println!("Done streaming into {}", file_path);

    Ok(())
}
```

## Nix

There is a Nix flake vailable which lets you work on the Rust create in a nix shell.

Just run the following command and you are in the business:

```shell
nix develop
```

# TODO

- [ ] gRPC streaming
- [ ] clean up the messy code
