use crate::error::Error;
use crate::prelude::*;
use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use reqwest::{Body, Method, Request, Response, Url};
use std::env;

const BASE_URL: &str = "https://api.play.ht/api";
const V2_PATH: &str = "v2";
// TODO: this is used for gRPC streaming
// Remove this attribute once implemented.
#[allow(unused)]
const V1_PATH: &str = "v1";
const USER_ID_HEADER: &str = "X-USER-ID";
const USER_AGENT: &str = "milosgajdos/playht_rs";

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    pub url: Url,
    pub headers: HeaderMap,
}

impl Client {
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
        let url = format!("{}/{}", self.url.unwrap(), path.into()).parse::<Url>()?;
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
                header::AUTHORIZATION.as_str(),
                HeaderValue::from_str(&secret_key).unwrap(),
            );
        }
        if let Ok(user_id) = env::var("PLAYHT_USER_ID") {
            headers.append(USER_ID_HEADER, HeaderValue::from_str(&user_id).unwrap());
        }
        headers.append(header::USER_AGENT, HeaderValue::from_static(USER_AGENT));

        let url = match format!("{}/{}", BASE_URL, V2_PATH).parse::<Url>() {
            Ok(url) => Some(url),
            Err(_) => None,
        };

        let client = reqwest::Client::new();

        Self {
            url,
            client: Some(client),
            headers: Some(headers),
        }
    }
}
