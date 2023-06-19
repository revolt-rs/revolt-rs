use super::{Client, Ratelimit};

use std::time::Duration;

use reqwest::{header::HeaderMap, ClientBuilder as RequestClientBuilder};

const USER_AGENT: &str = "revolt-rs (https://github.com/revolt-rs/revolt-rs)";

/// A builder for [`Client`].
#[derive(Debug)]
#[must_use = "This is useless if no Client is present"]
pub struct ClientBuilder {
    pub(crate) default_headers: Option<HeaderMap>,
    pub(crate) ratelimit: Option<Ratelimit>,
    pub(crate) base_url: Option<Box<str>>,
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
        let header_key = match self.is_bot {
            Some(true) => "x-bot-token",
            Some(false) => "x-session-token",
            _ => "x-bot-token",
        };

        let mut default_headers = HeaderMap::new();
        default_headers.insert(header_key, self.token.as_ref().unwrap().parse().unwrap());

        let client = RequestClientBuilder::new()
            .default_headers(default_headers)
            .user_agent(USER_AGENT)
            .timeout(self.http_timeout) 
            .build()
            .unwrap();

        Client {
            http: client,
            default_headers: self.default_headers,
            ratelimit: self.ratelimit,
            base_url: self.base_url,
            http_timeout: self.http_timeout,
            token: self.token,
            is_bot: self.is_bot,
        }
    }

    /// Set the token that is used for HTTP requests.
    pub fn token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    /// Set the type of header to determine weather the type of token is for a user or bot
    /// used in HTTP requests.
    pub fn is_bot(mut self, is_bot: Option<bool>) -> Self {
        self.is_bot = is_bot;
        self
    }

    /// Set the ratelimit for requests.
    pub fn ratelimit(mut self, ratelimit: Option<Ratelimit>) -> Self {
        self.ratelimit = ratelimit;

        self
    }

    /// Set the HTTP timeout for requests.
    pub fn http_timeout(mut self, timeout: Duration) -> Self {
        self.http_timeout = timeout;
        self
    }

    /// Set the default headers for requests.
    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.default_headers.replace(headers);
        self
    }
 
    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url.replace(base_url.into_boxed_str());
        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        #[allow(clippy::box_default)]
        Self {
            default_headers: None,
            ratelimit: None,
            base_url: Some("https://api.revolt.chat".into()),
            http_timeout: Duration::from_secs(10),
            token: None,
            is_bot: Some(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ClientBuilder;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ClientBuilder: Debug, Default, Send, Sync);

    #[test]
    fn client_debug() {
        assert!(format!("{:?}", ClientBuilder::new().token("foo".to_owned())).contains("foo"));
        assert!(format!("{:?}", ClientBuilder::new()).contains("None"));
    }
}
