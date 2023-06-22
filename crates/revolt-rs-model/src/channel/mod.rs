use serde::{Deserialize, Serialize};

use self::{channel_type::ChannelType, icon::ChannelIcon};

pub mod channel;
pub mod channel_type;
pub mod icon;
pub mod category;
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "channel_type")]
    pub channel_type: ChannelType,
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Id of the user this channel belongs to
    #[serde(rename = "user")]
    pub user: String,
    /// Whether this direct message channel is currently open on both sides
    #[serde(rename = "active")]
    pub active: bool,
    /// Array of user ids participating in channel
    #[serde(rename = "recipients")]
    pub recipients: Vec<String>,
    /// Id of the last message sent in this channel
    #[serde(rename = "last_message_id", skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<Option<String>>,
    /// Display name of the channel
    #[serde(rename = "name")]
    pub name: String,
    /// User id of the owner of the group
    #[serde(rename = "owner")]
    pub owner: String,
    /// Channel description
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<Box<ChannelIcon>>>,
    /// Permissions assigned to members of this group (does not apply to the owner of the group)
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Option<i64>>,
    /// Whether this channel is marked as not safe for work
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Id of the server this channel belongs to
    #[serde(rename = "server")]
    pub server: String,
    #[serde(rename = "default_permissions", skip_serializing_if = "Option::is_none")]
    pub default_permissions: Option<Option<Box<OverrideField>>>,
    /// Permissions assigned based on role to this channel
    #[serde(rename = "role_permissions", skip_serializing_if = "Option::is_none")]
    pub role_permissions: Option<::std::collections::HashMap<String, OverrideField>>,
}
