use serde::{Deserialize, Serialize};

pub mod channel_type;
pub mod message;
pub mod webhook;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChannelCompositeKey {
    /// Channel Id
    #[serde(rename = "channel")]
    pub channel: String,
    /// User Id
    #[serde(rename = "user")]
    pub user: String,
}
