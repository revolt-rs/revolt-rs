use std::time::Duration;

use hyper::HeaderMap;

use super::{Client, Ratelimit};

/// A builder for [`Client`].
#[derive(Debug)]
#[must_use = "This is useless if no Client is present"]
pub struct ClientBuilder {
    pub(crate) default_headers: Option<HeaderMap>,
    pub(crate) ratelimit: Option<Ratelimit>,
    pub(crate) proxy: Option<Box<str>>,
    pub(crate) http_timeout: Duration,
    pub(super) token: Option<String>,
    pub(super) is_bot: Option<bool>,
}

impl ClientBuilder {
    /// Create a new builder to create a [`Client`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the [`Client`]
    pub fn exec(self) -> Client {
        // TODO: establish a connection to the gateway.

        Client {
            default_headers: self.default_headers,
            ratelimit: self.ratelimit,
            proxy: self.proxy,
            http_timeout: self.http_timeout,
            token: self.token,
            is_bot: self.is_bot,
        }
    }

    pub fn token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn is_bot(self, is_bot: Option<bool>) -> Self { 
        let is_bot = match is_bot {
            Some(true) => "x-bot-token",
            Some(false) => "x-session-token",
            _ => "x-bot-token",
        };

        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        #[allow(clippy::box_default)]
        Self {
            default_headers: None,
            ratelimit: None,
            proxy: None,
            http_timeout: Duration::from_secs(10),
            token: None,
            is_bot: Some(true),
        }
    }
}
