use serde::Serialize;

use {
    futures_util::{SinkExt, StreamExt},
    std::time::{Duration, Instant},
    tokio::net::TcpStream,
    tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream},
    tracing::info,
};

#[cfg(not(feature = "msgpack"))]
const BONFIRE_URL: &str = "wss://ws.revolt.chat";
#[cfg(feature = "msgpack")]
const BONFIRE_URL: &str = "wss://ws.revolt.chat/?format=msgpack";

#[derive(Debug)]
pub struct WebSocketClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    heartbeat_interval: Duration,
    pong_timeout: Duration,
}

// This is temporary while I was testing the websocket connection.
#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Events {
    Authenticate {
        token: String,
    },
    BeginTyping {
        #[serde(rename = "channel")]
        channel_id: String,
    },
    EndTyping {
        #[serde(rename = "channel")]
        channel_id: String,
    },
    Ping {
        data: usize,
    },
}

impl WebSocketClient {
    pub async fn connect(&mut self, event: Events) -> Result<Self, ()> {
        let (stream, _) = connect_async(BONFIRE_URL).await?;
        let now = Instant::now();

        Ok(Self {
            stream,
            heartbeat_interval: Duration::from_secs(0),
            pong_timeout: Duration::from_secs(0),
        })
    }

    pub async fn send(&mut self, event: Events) -> Result<() ,()> {
        #[cfg(not(feature = "msgpack"))]
        let msg = Message::Text(serde_json::to_string(&event).unwrap());
        #[cfg(feature = "msgpack")]
        let msg = Message::Binary(rmp_serde::to_vec(&event).unwrap());

        self.stream.send(msg).await;

        Ok(())
    }

    pub async fn close(&mut self) -> Result<(), ()> {
        info!(target: "WS", "Closing the connection");

        self.stream.close(None).await;

        Ok(())
    }
}
