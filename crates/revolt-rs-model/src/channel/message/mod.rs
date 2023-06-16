use serde::{Deserialize, Serialize};

use self::{interactions::Interactions, masquerade::Masquerade};

pub mod embed;
pub mod interactions;
pub mod masquerade;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MessageSend {
    /// **This is deprecated and replaced by `Idempotency-Key`!**
    /// 
    /// Unique token to prevent duplicate message sending  
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Option<String>>,
    /// Message content to send
    #[serde(rename = "content", default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
    /// Attachments to include in message
    #[serde(rename = "attachments", default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Option<Vec<String>>>,
    /// Messages to reply to
    #[serde(rename = "replies", default, skip_serializing_if = "Option::is_none")]
    pub replies: Option<Option<Vec<crate::models::Reply>>>,
    /// Embeds to include in message  Text embed content contributes to the content length cap
    #[serde(rename = "embeds", default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Option<Vec<crate::models::SendableEmbed>>>,
    #[serde(rename = "masquerade", default, skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Option<Box<Masquerade>>>,
    #[serde(rename = "interactions", default, skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Option<Box<Interactions>>>,
}
