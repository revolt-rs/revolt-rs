use std::time::Duration;

use hyper::HeaderMap;
use revolt_rs_model::id::{Id, ServerMarker};

use self::builder::ClientBuilder;

mod builder;

const USER_AGENT: &str = "revolt-rs (https://github.com/revolt-rs/revolt-rs";

#[derive(Debug)]
pub struct Ratelimit {
    pub limit: u16,
    pub remaining: u16,
    pub reset_after: u64,
}

/// Revolt http client.
#[derive(Debug)]
pub struct Client {
    default_headers: Option<HeaderMap>,
    ratelimit: Option<Ratelimit>,
    proxy: Option<Box<str>>,
    http_timeout: Duration,
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
