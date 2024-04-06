# playht_rs

[![Build Status](https://github.com/milosgajdos/playht_rs/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/milosgajdos/playht_rs/actions?query=workflow%3ACI)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

An unofficial Play.ht Rust API client crate. Similar to the [Go module](https://github.com/milosgajdos/go-playht) implementation.

In order to use this create you must create an account with [play.ht](https://play.ht), generate an API secret and retrieve your User ID. See the official docs [here](https://docs.play.ht/reference/api-authentication).

# Get started

There are a few code samples available in the [examples](./src/bin) directory so please do have a look. They could give you some idea about how to use this crate.

> [!IMPORTANT]
> Before you attempt to run any of the samples you must set a couple of environment variables.
> These are automatically read by the client when it gets created; you can override them in your own code.

* `PLAYHT_SECRET_KEY`: API secret key
* `PLAYHT_USER_ID`: Play.HT User ID

## Nix

TBD

# Basics

TBD

# TODO

TBD
