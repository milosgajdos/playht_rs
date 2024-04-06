pub mod job;
pub mod tts;
pub mod voice;

use crate::{error::*, prelude::*};
use job::{TTSJob, TTSJobReq, TTS_JOB_PATH};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    multipart, Body, Method, Request, Response, Url,
};
use std::env;
use voice::{
    CloneVoiceFileRequest, CloneVoiceURLRequest, ClonedVoice, DeleteClonedVoiceRequest,
    DeleteClonedVoiceResp, Voice, CLONED_VOICES_INSTANT_PATH, CLONED_VOICES_PATH, VOICES_PATH,
};

const BASE_URL: &str = "https://api.play.ht/api";
const V2_PATH: &str = "/v2";
// TODO: this is used for gRPC streaming
// Remove this attribute once implemented.
#[allow(unused)]
const V1_PATH: &str = "/v1";

const USER_ID_HEADER: &str = "X-USER-ID";
const CLIENT_USER_AGENT: &str = "milosgajdos/playht_rs";

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    pub url: Url,
    pub headers: HeaderMap,
}

impl Client {
    pub fn new() -> Self {
        // NOTE: unwrap is warranted because default()
        // only sets up default configuration which
        // must contain valid client configurtion.
        let c = ClientBuilder::default().build().unwrap();

        c
    }

    pub fn remote_address(&self) -> String {
        let host = self.url.host().unwrap();
        let addr = format!("{}:{}", host, "443");

        addr
    }

    pub fn build_request<T: Into<Body>>(&self, method: Method, body: T) -> Result<Request> {
        let mut req_builder = self.client.request(method, self.url.clone());
        for (name, value) in &self.headers {
            req_builder = req_builder.header(name, value);
        }
        let req = req_builder.body(body).build()?;

        Ok(req)
    }

    pub async fn send_request(&self, req: Request) -> Result<Response> {
        let resp = self.client.execute(req).await?;

        Ok(resp)
    }

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
}

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
