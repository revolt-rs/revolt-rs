pub mod incoming;
pub mod interactions;
pub mod outgoing;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Format {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "msgpack")]
    Msgpack,
}

/// Gateway information containing the URL to connect to.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WebsocketGateway {
    /// URL to bonfire.
    pub url: String,
    /// Describes the protocol version in use.
    pub version: u8,
    /// Session token for authenticating the connecting user.
    pub token: Option<String>,
    /// In what format to send packets, default is JSON.
    pub format: Format,
}

impl Default for WebsocketGateway {
    fn default() -> WebsocketGateway {
        WebsocketGateway {
            url: String::from("wss://ws.revolt.chat"),
            version: 1,
            token: None,
            format: Format::Msgpack,
        }
    }
}
