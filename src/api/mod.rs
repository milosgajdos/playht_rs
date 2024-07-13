//! This module provides an implementation of play.ht API client.
//!
//! Use the [`Client`] for interfacing with the playht API.
//! You can build the [`Client`] with [`ClientBuilder`].
//! Note that the [`Client`] implementation is async. It leverages the [`tokio`]
//! runtime, so in order to use it you must do the same!
//!
//! There are a few submodules, each of which defines data structures
//! used by the [`Client`]. Each submodule also provides a simple one-off
//! functions that create one-off [`Client`] instance(s) underneath.
//! If you need just a simple one-off API call, such as when cloning a voice,
//! you might want to use those instead of creating the client first and calling
//! its appropriate method(s). You want to create your own instance of client if
//! you want to reuse it across different API calls instead of creating a new instance
//! for each separate API call.
//!

pub mod job;
pub mod stream;
pub mod tts;
pub mod voice;

use crate::{error::*, prelude::*};
use bytes::Bytes;
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
use tokio_stream::Stream;
use voice::{
    CloneVoiceFileRequest, CloneVoiceURLRequest, ClonedVoice, DeleteClonedVoiceRequest,
    DeleteClonedVoiceResp, Voice, CLONED_VOICES_INSTANT_PATH, CLONED_VOICES_PATH, VOICES_PATH,
};

/// API Base URL.
pub const BASE_URL: &str = "https://api.play.ht/api";
/// V2 API URL path.
const V2_PATH: &str = "/v2";
// TODO: this is used for gRPC streaming.
// Remove this attribute once implemented.
#[allow(unused)]
/// V1 API URL path.
const V1_PATH: &str = "/v1";

/// HTTP header used for API authentication.
/// See the the [official docs](https://docs.play.ht/reference/api-authentication).
pub const USER_ID_HEADER: &str = "X-USER-ID";
/// API client `User-Agent`.
pub const CLIENT_USER_AGENT: &str = "milosgajdos/playht_rs";

/// <https://play.ht> API client.
#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    url: Url,
    headers: HeaderMap,
}

/// Provides <https://play.ht> API client implementation.
impl Client {
    /// Creates a new client and returns it.
    /// It automatically initializes the client URL
    /// to the [play.ht](https://play.ht) API [`BASE_URL`].
    pub fn new() -> Self {
        // NOTE: unwrap is warranted because default()
        // only sets up default configuration which
        // must contain valid client configuration.
        let c = ClientBuilder::default().build().unwrap();

        c
    }

    /// Returns the remote host address as a string.
    /// The returned address has the following format: `host:port`.
    pub fn remote_address(&self) -> String {
        let host = self.url.host().unwrap();
        let addr = format!("{}:{}", host, "443");

        addr
    }

    /// Builds a request with a given `Method` and `body`.
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

    /// Sends the request to the remote API.
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

    /// Clones a voice clone from a file specified in the [`request`][voice::CloneVoiceFileRequest].
    /// See the [official docs](https://docs.play.ht/reference/api-create-instant-voice-clone).
    pub async fn clone_voice_from_file(&self, req: &CloneVoiceFileRequest) -> Result<ClonedVoice> {
        let voice_name_part = multipart::Part::text(req.voice_name.clone()).mime_str(TEXT_PLAIN)?;
        let sample_file_part = multipart::Part::bytes(std::fs::read(&req.sample_file)?)
            .file_name(req.sample_file.clone())
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

    /// Creates a voice clone from the URL specified in the [`request`][voice::CloneVoiceURLRequest].
    /// See the [official docs](https://docs.play.ht/reference/api-create-instant-voice-clone-via-file-url).
    pub async fn clone_voice_from_url(&self, req: &CloneVoiceURLRequest) -> Result<ClonedVoice> {
        let body = serde_json::to_string(req)?;
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

    /// Deletes a cloned voice.
    /// See the [official docs](https://docs.play.ht/reference/api-delete-cloned-voices).
    pub async fn delete_cloned_voice(
        &self,
        req: &DeleteClonedVoiceRequest,
    ) -> Result<DeleteClonedVoiceResp> {
        let body = serde_json::to_string(req)?;
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

    /// Creates an async TTS job and returns it.
    /// See the [official docs](https://docs.play.ht/reference/api-generate-audio).
    pub async fn create_tts_job(&self, req: &TTSJobReq) -> Result<TTSJob> {
        let body = serde_json::to_string(req)?;
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

    /// Creates an async TTS job and immediately writes its progress to the given writer.
    /// See the [official docs](https://docs.play.ht/reference/api-generate-audio).
    /// NOTE: The stream written into the writer contains SSE events
    /// reporting the progress of the job.
    pub async fn create_tts_job_write_progress_stream<W>(
        &self,
        w: &mut W,
        req: &TTSJobReq,
    ) -> Result<Option<String>>
    where
        W: tokio::io::AsyncWriteExt + Unpin,
    {
        let body = serde_json::to_string(req)?;
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

    /// Fetches the TTS job and returns it.
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

    /// Writes the progress stream the TTS job with the given id into the given writer.
    /// Unlike [`Client::create_tts_job_with_progress_stream`] this method does NOT
    /// create a new job, but merely writes the SSE events stream into the given writer.
    /// See the [official docs](https://docs.play.ht/reference/api-get-tts-data).
    pub async fn write_tts_job_progress_stream<W>(&self, w: &mut W, id: String) -> Result<()>
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

    /// Streams the TTS job progress.
    /// Unlike [`Client::write_tts_job_progress_stream`] this returns the TTS jobs
    /// progress stream to the consumer. The stream streams SSE events reporting
    /// the TTS job progress.
    pub async fn stream_tts_job_progress(
        &self,
        id: String,
    ) -> Result<impl Stream<Item = StreamResult<Bytes>>> {
        let tts_job_url = format!("{}{}/{}", self.url.as_str(), TTS_JOB_PATH, id);
        let resp = self
            .client
            .get(tts_job_url)
            .headers(self.headers.clone())
            .header(ACCEPT, TEXT_EVENT_STREAM)
            .send()
            .await?;

        Ok(resp.bytes_stream())
    }

    /// Write the audio stream of the TTS job with the given id into the given writer.
    /// Unlike [`Client::stream_tts_job_progress`] this method does not stream the SSE events
    /// reporting the progress of the async job that had been create, but rather
    /// it automatically streams the raw audio data to the given writer.
    /// See the [official docs](https://docs.play.ht/reference/api-get-tts-data).
    pub async fn write_tts_job_audio_stream<W>(&self, w: &mut W, id: String) -> Result<()>
    where
        W: tokio::io::AsyncWriteExt + Unpin,
    {
        let tts_job_url = format!("{}{}/{}", self.url.as_str(), TTS_JOB_PATH, id);
        let mut resp = self
            .client
            .get(tts_job_url)
            .headers(self.headers.clone())
            .send()
            .await?;

        while let Some(chunk) = resp.chunk().await? {
            w.write_all(&chunk).await?;
        }

        Ok(())
    }

    /// Writes TTS audio stream into the given writer.
    /// Unlike [`Client::write_tts_job_audio_stream`] this does not create an async job.
    /// Instead it immediately starts writing raw audio data into the given writer.
    /// See the [official docs](https://docs.play.ht/reference/api-generate-tts-audio-stream).
    pub async fn write_audio_stream<W>(&self, w: &mut W, req: &TTSStreamReq) -> Result<()>
    where
        W: tokio::io::AsyncWriteExt + Unpin,
    {
        let body = serde_json::to_string(req)?;
        let tts_stream_url = format!("{}{}", self.url.as_str(), TTS_STREAM_PATH);

        let mut resp = self
            .client
            .post(tts_stream_url)
            .body(body)
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()
            .await?;

        while let Some(chunk) = resp.chunk().await? {
            w.write_all(&chunk).await?;
        }

        Ok(())
    }

    /// Fetches audio stream URL instead of streaming raw audio like [`Client::stream_audio`].
    /// You can use the returned URL for streaming the raw audio.
    /// See the [official docs](https://docs.play.ht/reference/api-generate-tts-audio-stream).
    pub async fn get_audio_stream_url(&self, req: &TTSStreamReq) -> Result<TTSStreamURL> {
        let body = serde_json::to_string(req)?;
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

    /// Streams raw TTS audio.
    /// Unlike [`Client::write_audio_stream`] this method returns an async stream object
    /// that streams raw audio data. This way the consumer is in control of streaming.
    /// See the [official docs](https://docs.play.ht/reference/api-generate-tts-audio-stream).
    pub async fn stream_audio(
        &self,
        req: &TTSStreamReq,
    ) -> Result<impl Stream<Item = StreamResult<Bytes>>> {
        let body = serde_json::to_string(req)?;
        let tts_stream_url = format!("{}{}", self.url.as_str(), TTS_STREAM_PATH);

        let resp = self
            .client
            .post(tts_stream_url)
            .body(body)
            .headers(self.headers.clone())
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .send()
            .await?;

        Ok(resp.bytes_stream())
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
