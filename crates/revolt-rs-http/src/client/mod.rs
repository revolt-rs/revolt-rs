use hyper::HeaderMap;

use self::builder::{ClientBuilder, TokenType};

mod builder;

/// Revolt http client.
#[derive(Debug)]
pub struct Client {
    default_headers: Option<HeaderMap>,
    token: Option<String>,
}

impl Client {
    pub fn new(token: String) -> Self {
        ClientBuilder::default().token(token, TokenType::Bot).exec()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}
