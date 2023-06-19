use serde::{Deserialize, Serialize};

use self::{
    interactions::Interactions, masquerade::Masquerade, embed::{SendableEmbed, Embed}, reply::Reply, system_message::SystemMessageType
};

use super::webhook::MessageWebhook;

pub mod embed;
pub mod interactions;
pub mod reply;
pub mod masquerade;
pub mod system_message;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Message {
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Unique value generated by client sending this message
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Option<String>>,
    /// Id of the channel this message was sent in
    #[serde(rename = "channel")]
    pub channel: String,
    /// Id of the user or webhook that sent this message
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "webhook", default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Option<Box<MessageWebhook>>>,
    /// Message content
    #[serde(rename = "content", default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
    #[serde(rename = "system", default, skip_serializing_if = "Option::is_none")]
    pub system: Option<Option<Box<SystemMessageType>>>,
    /// Array of attachments
    #[serde(rename = "attachments", default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Option<Vec<crate::attachment::File>>>,
    /// ISO8601 formatted timestamp
    #[serde(rename = "edited", default, skip_serializing_if = "Option::is_none")]
    pub edited: Option<Option<String>>,
    /// Attached embeds to this message
    #[serde(rename = "embeds", default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Option<Vec<Embed>>>,
    /// Array of user ids mentioned in this message
    #[serde(rename = "mentions", default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Option<Vec<String>>>,
    /// Array of message ids this message is replying to
    #[serde(rename = "replies", default, skip_serializing_if = "Option::is_none")]
    pub replies: Option<Option<Vec<String>>>,
    /// Hashmap of emoji IDs to array of user IDs
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "interactions", skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Box<Interactions>>,
    #[serde(rename = "masquerade", default, skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Option<Box<Masquerade>>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct PartialMessage {
   /// Unique Id
    #[serde(rename = "_id", default, skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    /// Unique value generated by client sending this message
    #[serde(rename = "nonce", default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Option<String>>,
    /// Id of the channel this message was sent in
    #[serde(rename = "channel", default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// Id of the user or webhook that sent this message
    #[serde(rename = "author", default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "webhook", default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Option<Box<MessageWebhook>>>,
    /// Message content
    #[serde(rename = "content", default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
    #[serde(rename = "system", default, skip_serializing_if = "Option::is_none")]
    pub system: Option<Option<Box<SystemMessageType>>>,
    /// Array of attachments
    #[serde(rename = "attachments", default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Option<Vec<crate::attachment::File>>>,
    /// ISO8601 formatted timestamp
    #[serde(rename = "edited", default, skip_serializing_if = "Option::is_none")]
    pub edited: Option<Option<String>>,
    /// Attached embeds to this message
    #[serde(rename = "embeds", default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Option<Vec<Embed>>>,
    /// Array of user ids mentioned in this message
    #[serde(rename = "mentions", default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Option<Vec<String>>>,
    /// Array of message ids this message is replying to
    #[serde(rename = "replies", default, skip_serializing_if = "Option::is_none")]
    pub replies: Option<Option<Vec<String>>>,
    /// Hashmap of emoji IDs to array of user IDs
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "interactions", skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Box<Interactions>>,
    #[serde(rename = "masquerade", default, skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Option<Box<Masquerade>>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataMessageSend {
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
    pub replies: Option<Option<Vec<Reply>>>,
    /// Embeds to include in message  Text embed content contributes to the content length cap
    #[serde(rename = "embeds", default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Option<Vec<SendableEmbed>>>,
    #[serde(rename = "masquerade", default, skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Option<Box<Masquerade>>>,
    #[serde(rename = "interactions", default, skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Option<Box<Interactions>>>,
}
