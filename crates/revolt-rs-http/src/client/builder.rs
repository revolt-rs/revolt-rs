use hyper::{HeaderMap, http::HeaderValue};
use tracing::{event, Level};

use super::Client;

/// A builder for [`Client`].
#[derive(Debug)]
#[must_use = "This is useless if no Client is present"]
pub struct ClientBuilder {
    pub(crate) default_headers: Option<HeaderMap>,
    pub(super) token: Option<String>,
}

pub enum TokenType {
    User,
    Bot
}

impl ClientBuilder {
    /// Create a new builder to create a [`Client`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the [`Client`]
    pub fn exec(self) -> Client {
        // TODO: establish a connection to the gateway.

        event!(Level::DEBUG, "Something has happened!");

        Client {
            default_headers: self.default_headers,
            token: self.token,
        }
    }

    pub fn token(self, token: String, token_type: TokenType) -> Self {
        let mut headers = HeaderMap::new();

        match token_type {
            TokenType::User => {
                headers.insert("x-session-token", HeaderValue::from_str(&token).unwrap());
            },
            TokenType::Bot => {
                headers.insert("x-bot-token", HeaderValue::from_str(&token).unwrap());
            }
        }

        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        #[allow(clippy::box_default)]
        Self {
            token: None,
            default_headers: None,
        }
    }
}
