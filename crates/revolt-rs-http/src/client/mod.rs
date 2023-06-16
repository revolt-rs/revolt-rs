use hyper::HeaderMap;

use self::builder::ClientBuilder;

mod builder;

/// Revolt http client.
#[derive(Debug)]
pub struct Client {
    default_headers: Option<HeaderMap>,
    token: Option<String>,
    is_bot: bool,
}

impl Client {
    pub fn new(token: String, is_bot: bool) -> Self {
        ClientBuilder::default().token(token, is_bot).exec()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }
}
