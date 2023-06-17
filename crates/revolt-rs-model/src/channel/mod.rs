use serde::{Deserialize, Serialize};

pub mod channel_type;
pub mod message;
pub mod webhook;

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OverrideField {
    /// Allow bit flags
    #[serde(rename = "a")]
    pub a: i64,
    /// Disallow bit flags
    #[serde(rename = "d")]
    pub d: i64,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChannelCompositeKey {
    /// Channel Id
    #[serde(rename = "channel")]
    pub channel: String,
    /// User Id
    #[serde(rename = "user")]
    pub user: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct ChannelIcon {
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Tag / bucket this file was uploaded to
    #[serde(rename = "tag")]
    pub tag: String,
    /// Original filename
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::attachment::Metadata>,
    /// Raw content type of this file
    #[serde(rename = "content_type")]
    pub content_type: String,
    /// Size of this file (in bytes)
    #[serde(rename = "size")]
    pub size: i32,
    /// Whether this file was deleted
    #[serde(rename = "deleted", default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Option<bool>>,
    /// Whether this file was reported
    #[serde(rename = "reported", default, skip_serializing_if = "Option::is_none")]
    pub reported: Option<Option<bool>>,
    #[serde(rename = "message_id", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Option<String>>,
    #[serde(rename = "user_id", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Option<String>>,
    #[serde(rename = "server_id", default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<Option<String>>,
    /// Id of the object this file is associated with
    #[serde(rename = "object_id", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<Option<String>>,
}