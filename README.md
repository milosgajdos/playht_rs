# playht_rs

[![Build Status](https://github.com/milosgajdos/playht_rs/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/milosgajdos/playht_rs/actions?query=workflow%3ACI)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

An unofficial [play.ht](https://play.ht) Rust API client crate. Similar to the [Go module](https://github.com/milosgajdos/go-playht) implementation.

In order to use this create you must create an account on [play.ht](https://play.ht), generate an API secret and retrieve your User ID.
See the official docs [here](https://docs.play.ht/reference/api-authentication).

# Get started

> [!IMPORTANT]
> Before you attempt to run any of the samples you must set a couple of environment variables.
> These are automatically read by the client when it gets created; you can override them in your own code.

* `PLAYHT_SECRET_KEY`: API secret key
* `PLAYHT_USER_ID`: Play.HT User ID

Check the crate:
```
cargo check
```

Build the crate:
```shell
cargo build
```

## Examples

There are a few code samples available in the [examples](./src/bin) directory so please do have a look. They could give you some idea about how to use this crate.

Run one of the examples:
```shell
cargo run --bin get_voices
```

The above example calls the program that lists all available [play.ht](https://play.ht) stock voices:
```rust
use playht_rs::{
    api::{self, voice::get_stock_voices},
    prelude::*,
};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // use the api::voice module
    let voices = get_stock_voices().await?;
    println!("Got {} voices", voices.len());

    // use the api module directly
    let voices = api::Client::new().get_stock_voices().await?;
    println!("Got {} voices", voices.len());

    Ok(())
}
```

## Nix

There is a Nix flake file vailable which lets you work on the Rust create in nix shell.

Just run the following command and you are in the business:

```shell
nix develop
```

# Basics

There are two ways to create audio/speech from the text using the API:
* Job: audio generation is done in async; when you create a job you can monitor its progress via [SSE](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events)
* Stream: a real-time audio stream available immediately as soon as the stream has been created via the API

The API also allows you to clone a voice using a small sample of limited size. See the [docs](https://docs.play.ht/reference/api-create-instant-voice-clone).

# TODO

* [ ] gRPC streaming
* [ ] clean up the messy code
