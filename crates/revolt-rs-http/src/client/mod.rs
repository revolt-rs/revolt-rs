use std::time::Duration;

use crate::error::Errors;

use self::builder::ClientBuilder;

use reqwest::{header::HeaderMap, Client as ReqwestClient};
use revolt_rs_model::errors::ErrorType;

mod builder;

type Result<T> = std::result::Result<T, Errors>;

#[async_trait::async_trait]
pub trait ResponseExt {
    async fn process_error(self) -> Result<Self>
    where
        Self: Sized;
}

#[async_trait::async_trait]
impl ResponseExt for reqwest::Response {
    async fn process_error(self) -> Result<Self>
    where
        Self: Sized,
    {
        match self.status().as_u16() {
            200..=299 => Ok(self),
            401 => Err(Errors::Api(ErrorType::InvalidOperation)),
            404 => Err(Errors::Api(ErrorType::NotFound)),
            _ => Err(Errors::Api(self.json().await?)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ratelimit {
    pub limit: u16,
    pub remaining: u16,
    pub reset_after: u64,
}

/// Revolt http client.
#[derive(Debug, Clone)]
pub struct Client {
    default_headers: Option<HeaderMap>,
    ratelimit: Option<Ratelimit>,
    pub base_url: Option<Box<str>>,
    http_timeout: Duration,
    pub http: ReqwestClient,
    token: Option<String>,
    is_bot: Option<bool>,
}

impl Client {
    /// Create a new client with a token.
    pub fn new(token: String, is_bot: Option<bool>) -> Self {
        ClientBuilder::default().token(token).is_bot(is_bot.is_some()).exec()
    }

    /// Create a new builder to create a client.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}
