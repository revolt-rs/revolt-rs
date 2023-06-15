use futures_util::SinkExt;
use serde::Serialize;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tracing::info;

#[derive(Debug)]
pub struct WebSocketClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    heartbeat_interval: Duration,
    pong_timeout: Duration, 
    // websocket_domain: String,
    // token: String,
}

const BONFIRE_URL: &str = "wss://ws.revolt.chat/?format=msgpack";

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

// TOOD: support self-hosted revolt instances.
impl WebSocketClient {
    pub async fn connect() -> Result<WebSocketClient, Box<dyn std::error::Error + Send + Sync>> {
        // let _ = self.close().await;

        let (stream, _) = connect_async(BONFIRE_URL)
            .await
            .expect("Connection to websocket failed");

        Ok(Self {
            stream,
            heartbeat_interval: Duration::from_secs(30),
            pong_timeout: Duration::from_secs(10),
        })
    }

    pub async fn send(&mut self, event: Events) -> Result<(), ()> {
        let msg = Message::Binary(rmp_serde::to_vec(&event).unwrap());

        let _ = self.stream.send(msg).await;

        Ok(())
    }

    pub async fn close(&mut self) -> Result<(), ()> {
        info!(target: "WS", "Closing the connection");

        let _ = self.stream.close(None).await;

        Ok(())
    }
}