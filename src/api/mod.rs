//! This module provides an implementation of play.ht API client.
//!
//! Use the [`Client`] for interacing with the playht API.
//! You can configure and build the [`Client`] with [`ClientBuilder`].
//! Note that the [`Client`] implementation is async -- it leverages [`tokio`] runtime,
//! so in order to use it you must do the same.
//!
//! There are a few submodules, each of which defines various data structures
//! leveraged by the [`Client`]. Each module also provides a simple oone-off
//! functions that create the one-off instance(s) of the [`Client`] underneath.
//! If you need just a specific functionality, such as cloning a voice,
//!  you might want to use those modules instead of creating the client, etc.
//!

pub mod job;
pub mod stream;
pub mod tts;
pub mod voice;

use crate::{error::*, prelude::*};
use job::{TTSJob, TTSJobReq, TTS_JOB_PATH};
use reqwest::{
    header::{
        HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_LOCATION, CONTENT_TYPE,
        USER_AGENT,
    },
    multipart, Body, Method, Request, Response, Url,
};
use std::env;
use stream::{TTSStreamReq, TTSStreamURL, TTS_STREAM_PATH};
use voice::{
    CloneVoiceFileRequest, CloneVoiceURLRequest, ClonedVoice, DeleteClonedVoiceRequest,
    DeleteClonedVoiceResp, Voice, CLONED_VOICES_INSTANT_PATH, CLONED_VOICES_PATH, VOICES_PATH,
};

/// API Base URL.
pub const BASE_URL: &str = "https://api.play.ht/api";
/// V2 API URL path.
const V2_PATH: &str = "/v2";
// TODO: this is used for gRPC streaming
// Remove this attribute once implemented.
#[allow(unused)]
/// V1 API URL path.
const V1_PATH: &str = "/v1";

/// HTTP header used for API authentication.
pub const USER_ID_HEADER: &str = "X-USER-ID";
/// API client `User-Agent`.
pub const CLIENT_USER_AGENT: &str = "milosgajdos/playht_rs";

/// <https://play.ht> API client.
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    pub url: Url,
    pub headers: HeaderMap,
}

/// Provides <https://play.ht> API client implementation.
impl Client {
    /// Create a new client and return it.
    /// It automatically initializes the client URL
    /// to the [play.ht](https://play.ht) API [`BASE_URL`].
    pub fn new() -> Self {
        // NOTE: unwrap is warranted because default()
        // only sets up default configuration which
        // must contain valid client configurtion.
        let c = ClientBuilder::default().build().unwrap();

        c
    }

    /// Return the remote host address as a string.
    /// The returned address has the following format: `host:port`.
    pub fn remote_address(&self) -> String {
        let host = self.url.host().unwrap();
        let addr = format!("{}:{}", host, "443");

        addr
    }

    /// Build a request with a given `Method` and `body`.
    /// The reeturned request can then be passed to [`Client::send_request`].
    /// Generally, we recommend using one of the [`Client`] methods
    /// which builds the request for you automatically, but sometimes
    /// this might come in handy, such as when a new API endpoint
    /// is added and you don't want to wait for it to be added to this crate.
    pub fn build_request<T: Into<Body>>(&self, method: Method, body: T) -> Result<Request> {
        let mut req_builder = self.client.request(method, self.url.clone());
        for (name, value) in &self.headers {
            req_builder = req_builder.header(name, value);
        }
        let req = req_builder.body(body).build()?;

        Ok(req)
    }

    /// Send the request to the remote API.
    /// We recommend using one of the specific [`Client`] methods,
    /// but as with [`Client::build_request`], this can be handy in some situations.
    pub async fn send_request(&self, req: Request) -> Result<Response> {
        let resp = self.client.execute(req).await?;

        Ok(resp)
    }

    /// Returns all available stock voices.
    /// See the [official docs](https://docs.play.ht/reference/api-list-ultra-realistic-voices).
    pub async fn get_stock_voices(&self) -> Result<Vec<Voice>> {
        let voices_url = format!("{}{}", self.url.as_str(), VOICES_PATH);
        let resp = self
            .client
            .get(voices_url)
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()
            .await?;

        if resp.status().is_success() {
            let voices: Vec<Voice> = resp.json().await?;
            return Ok(voices);
        }

        let api_error: APIError = resp.json().await?;
        Err(Box::new(Error::APIError(api_error)))
    }

    /// Returns all cloned voices.
    /// See the [official docs](https://docs.play.ht/reference/api-list-cloned-voices);
    pub async fn get_cloned_voices(&self) -> Result<Vec<ClonedVoice>> {
        let voices_url = format!("{}{}", self.url.as_str(), CLONED_VOICES_PATH);
        let resp = self
            .client
            .get(voices_url)
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()
            .await?;

        if resp.status().is_success() {
            let voices: Vec<ClonedVoice> = resp.json().await?;
            return Ok(voices);
        }

        let api_error: APIError = resp.json().await?;
        Err(Box::new(Error::APIError(api_error)))
    }

    /// Clone a voice clone from a file specified in the [`request`][voice::CloneVoiceFileRequest].
    /// Seee the [official docs](https://docs.play.ht/reference/api-create-instant-voice-clone).
    pub async fn clone_voice_from_file(&self, req: CloneVoiceFileRequest) -> Result<ClonedVoice> {
        let voice_name_part = multipart::Part::text(req.voice_name).mime_str(TEXT_PLAIN)?;
        let sample_file_part = multipart::Part::bytes(std::fs::read(&req.sample_file)?)
            .file_name(req.sample_file)
            .mime_str(&req.mime_type)?;

        let form = multipart::Form::new()
            .part("voice_name", voice_name_part)
            .part("sample_file", sample_file_part);

        let clone_voice_url = format!("{}{}", self.url.as_str(), CLONED_VOICES_INSTANT_PATH);
        let resp = self
            .client
            .post(clone_voice_url)
            .headers(self.headers.clone())
            .header(ACCEPT, APPLICATION_JSON)
            .header(
                CONTENT_TYPE,
                format!("{}; boundary={}", MULTIPART_FORM, form.boundary()),
            )
            .multipart(form)
            .send()
            .await?;

        if resp.status().is_success() {
            let voice: ClonedVoice = resp.json().await?;
            return Ok(voice);
        }

        let api_error: APIError = resp.json().await?;
        Err(Box::new(Error::APIError(api_error)))
    }

    /// Create a voice clone from a URL specified in the [`request`][voice::CloneVoiceURLRequest].
    /// See the [official docs](https://docs.play.ht/reference/api-create-instant-voice-clone-via-file-url).
    pub async fn clone_voice_from_url(&self, req: CloneVoiceURLRequest) -> Result<ClonedVoice> {
        let body = serde_json::to_string(&req)?;
        let clone_voice_url = format!("{}{}", self.url.as_str(), CLONED_VOICES_PATH);
        let resp = self
            .client
            .post(clone_voice_url)
            .headers(self.headers.clone())
            .header(ACCEPT, APPLICATION_JSON)
            .body(body)
            .send()
            .await?;

        if resp.status().is_success() {
            let voice: ClonedVoice = resp.json().await?;
            return Ok(voice);
        }

        let api_error: APIError = resp.json().await?;
        Err(Box::new(Error::APIError(api_error)))
    }

    /// Delete a cloned voice.
    /// See the [official docs](https://docs.play.ht/reference/api-delete-cloned-voices).
    pub async fn delete_cloned_voice(
        &self,
        req: DeleteClonedVoiceRequest,
    ) -> Result<DeleteClonedVoiceResp> {
        let body = serde_json::to_string(&req)?;
        let clone_voice_url = format!("{}{}", self.url.as_str(), CLONED_VOICES_PATH);
        let resp = self
            .client
            .delete(clone_voice_url)
            .body(body)
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .header(ACCEPT, APPLICATION_JSON)
            .send()
            .await?;

        if resp.status().is_success() {
            let del_resp: DeleteClonedVoiceResp = resp.json().await?;
            return Ok(del_resp);
        }

        let api_error: APIError = resp.json().await?;
        Err(Box::new(Error::APIError(api_error)))
    }

    /// Create an async TTS job and return its metadata.
    /// See the [official docs](https://docs.play.ht/reference/api-generate-audio).
    pub async fn create_tts_job(&self, req: TTSJobReq) -> Result<TTSJob> {
        let body = serde_json::to_string(&req)?;
        let tts_job_url = format!("{}{}", self.url.as_str(), TTS_JOB_PATH);
        let resp = self
            .client
            .post(tts_job_url)
            .body(body)
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .header(ACCEPT, APPLICATION_JSON)
            .send()
            .await?;

        if resp.status().is_success() {
            let tts_job: TTSJob = resp.json().await?;
            return Ok(tts_job);
        }

        let api_error: APIError = resp.json().await?;
        Err(Box::new(Error::APIError(api_error)))
    }

    /// Create an async TTS job and immediately stream the progres into the given writer.
    /// See the [official docs](https://docs.play.ht/reference/api-generate-audio).
    /// NOTE: The stream written into the given writer contains SSE events
    /// reporting the progress of the async job.
    pub async fn create_tts_job_with_progress_stream<W>(
        &self,
        w: &mut W,
        req: TTSJobReq,
    ) -> Result<Option<String>>
    where
        W: tokio::io::AsyncWriteExt + Unpin,
    {
        let body = serde_json::to_string(&req)?;
        let tts_job_url = format!("{}{}", self.url.as_str(), TTS_JOB_PATH);
        let mut resp = self
            .client
            .post(tts_job_url)
            .body(body)
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .header(ACCEPT, TEXT_EVENT_STREAM)
            .send()
            .await?;

        let stream_url = resp
            .headers()
            .get(CONTENT_LOCATION)
            .and_then(|hv| hv.to_str().ok().map(|s| s.to_string()));

        while let Some(chunk) = resp.chunk().await? {
            w.write_all(&chunk).await?;
        }

        Ok(stream_url)
    }

    /// Query the async TTS job and return it.
    /// See the [official docs](https://docs.play.ht/reference/api-get-tts-data).
    pub async fn get_tts_job(&self, id: String) -> Result<TTSJob> {
        let tts_job_url = format!("{}{}/{}", self.url.as_str(), TTS_JOB_PATH, id);
        let resp = self
            .client
            .get(tts_job_url)
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()
            .await?;

        if resp.status().is_success() {
            let tts_job: TTSJob = resp.json().await?;
            return Ok(tts_job);
        }

        let api_error: APIError = resp.json().await?;
        Err(Box::new(Error::APIError(api_error)))
    }

    /// Stream the progress of the TTS job with the given id.
    /// Unlike [`Client::create_tts_job_with_progress_stream`] this doe NOT
    /// create a new TTS job, but merely streams the SSE events about its progress.
    /// See the [official docs](https://docs.play.ht/reference/api-get-tts-data).
    pub async fn stream_tts_job_progress<W>(&self, w: &mut W, id: String) -> Result<()>
    where
        W: tokio::io::AsyncWriteExt + Unpin,
    {
        let tts_job_url = format!("{}{}/{}", self.url.as_str(), TTS_JOB_PATH, id);
        let mut resp = self
            .client
            .get(tts_job_url)
            .headers(self.headers.clone())
            .header(ACCEPT, TEXT_EVENT_STREAM)
            .send()
            .await?;

        while let Some(chunk) = resp.chunk().await? {
            w.write_all(&chunk).await?;
        }

        Ok(())
    }

    /// Stream the audio from the TTS job in real time.
    /// Unlike [`Client::stream_tts_job_progress`] this does not stream the SSE events
    /// reporting the progress of the async job that had been create, but rather
    /// it automatically streams the raw audio data to the given writer.
    /// See the [official docs](https://docs.play.ht/reference/api-get-tts-data).
    pub async fn stream_tts_job_audio<W>(&self, w: &mut W, id: String) -> Result<()>
    where
        W: tokio::io::AsyncWriteExt + Unpin,
    {
        let tts_job_url = format!("{}{}/{}", self.url.as_str(), TTS_JOB_PATH, id);
        let mut resp = self
            .client
            .get(tts_job_url)
            .headers(self.headers.clone())
            .header(ACCEPT, AUDIO_MPEG)
            .send()
            .await?;

        while let Some(chunk) = resp.chunk().await? {
            w.write_all(&chunk).await?;
        }

        Ok(())
    }

    /// Stream TTS audio in real-time to the given writer.
    /// Unlike [`Client::stream_tts_job_audio`] this does not create an async job.
    /// Instead it immediately starts streaming the audio data into the given writer.
    /// See the [official docs](https://docs.play.ht/reference/api-generate-tts-audio-stream).
    pub async fn stream_audio<W>(&self, w: &mut W, req: TTSStreamReq) -> Result<()>
    where
        W: tokio::io::AsyncWriteExt + Unpin,
    {
        let body = serde_json::to_string(&req)?;
        let tts_stream_url = format!("{}{}", self.url.as_str(), TTS_STREAM_PATH);

        let mut resp = self
            .client
            .post(tts_stream_url)
            .body(body)
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .header(ACCEPT, AUDIO_MPEG)
            .send()
            .await?;

        while let Some(chunk) = resp.chunk().await? {
            w.write_all(&chunk).await?;
        }

        Ok(())
    }

    /// Get the audio stream URL instead of streaming the raw audio like [`Client::stream_audio`].
    /// You can then use the returned URL for fetching the stream.
    /// See the [official docs](https://docs.play.ht/reference/api-generate-tts-audio-stream).
    pub async fn get_audio_stream_url(&self, req: TTSStreamReq) -> Result<TTSStreamURL> {
        let body = serde_json::to_string(&req)?;
        let tts_stream_url = format!("{}{}", self.url.as_str(), TTS_STREAM_PATH);

        let resp = self
            .client
            .post(tts_stream_url)
            .body(body)
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .header(ACCEPT, APPLICATION_JSON)
            .send()
            .await?;

        if resp.status().is_success() {
            let audio_stream_url: TTSStreamURL = resp.json().await?;
            return Ok(audio_stream_url);
        }

        let api_error: APIError = resp.json().await?;
        Err(Box::new(Error::APIError(api_error)))
    }
}

/// Configures and builds the [`Client`].
#[derive(Debug)]
pub struct ClientBuilder {
    client: Option<reqwest::Client>,
    url: Option<Url>,
    headers: Option<HeaderMap>,
}

impl ClientBuilder {
    pub fn new() -> Result<Self> {
        let cb = ClientBuilder::default();

        Ok(cb)
    }

    pub fn header(mut self, name: &str, value: &str) -> Result<Self> {
        let header_name = name.parse::<HeaderName>()?;
        let header_value = value.parse::<HeaderValue>()?;
        self.headers
            .as_mut()
            .unwrap()
            .insert(header_name, header_value);

        Ok(self)
    }

    pub fn path(mut self, path: impl Into<String>) -> Result<Self> {
        let url = format!("{}{}", self.url.unwrap(), path.into()).parse::<Url>()?;
        self.url = Some(url);

        Ok(self)
    }

    pub fn req_client<T: Into<reqwest::Client>>(mut self, client: T) -> Result<Self> {
        self.client = Some(client.into());

        Ok(self)
    }

    pub fn build(self) -> Result<Client> {
        let Some(url) = self.url else {
            return Err(Box::new(Error::ClientBuildError(
                "url is not set".to_string(),
            )));
        };

        Ok(Client {
            url,
            client: self.client.unwrap(),
            headers: self.headers.unwrap(),
        })
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        let mut headers = HeaderMap::new();
        if let Ok(secret_key) = env::var("PLAYHT_SECRET_KEY") {
            headers.append(
                AUTHORIZATION.as_str(),
                HeaderValue::from_str(&secret_key).unwrap(),
            );
        }
        if let Ok(user_id) = env::var("PLAYHT_USER_ID") {
            headers.append(USER_ID_HEADER, HeaderValue::from_str(&user_id).unwrap());
        }
        headers.append(USER_AGENT, HeaderValue::from_static(CLIENT_USER_AGENT));

        let url = format!("{}{}", BASE_URL, V2_PATH).parse::<Url>().ok();

        let client = reqwest::Client::new();

        Self {
            url,
            client: Some(client),
            headers: Some(headers),
        }
    }
}
