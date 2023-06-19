use std::time::Duration;

use self::builder::ClientBuilder;

use reqwest::{header::HeaderMap, Client as ReqwestClient};

mod builder;

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
        ClientBuilder::default().token(token).is_bot(is_bot).exec()
    }

    /// Create a new builder to create a client.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}
