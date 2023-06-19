use serde::{Deserialize, Serialize};

use crate::channel::category::Category;

use self::{
    banner::ServerBanner, icon::ServerIcon, role::Role, system_messages::ServerSystemMessages,
};

pub mod banner;
pub mod icon;
pub mod member;
pub mod role;
pub mod permissions;
pub mod bans;
pub mod system_messages;
pub mod emoji;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Server {
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// User id of the owner
    #[serde(rename = "owner")]
    pub owner: String,
    /// Name of the server
    #[serde(rename = "name")]
    pub name: String,
    /// Description for the server
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Channels within this server
    #[serde(rename = "channels")]
    pub channels: Vec<String>,
    /// Categories for this server
    #[serde(rename = "categories", default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Option<Vec<Category>>>,
    #[serde(rename = "system_messages", default, skip_serializing_if = "Option::is_none")]
    pub system_messages: Option<Option<Box<ServerSystemMessages>>>,
    /// Roles for this server
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<::std::collections::HashMap<String, Role>>,
    /// Default set of server and channel permissions
    #[serde(rename = "default_permissions")]
    pub default_permissions: i64,
    #[serde(rename = "icon", default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<Box<ServerIcon>>>,
    #[serde(rename = "banner", default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<Option<Box<ServerBanner>>>,
    /// Bitfield of server flags
    #[serde(rename = "flags", default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Option<i32>>,
    /// Whether this server is flagged as not safe for work
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    /// Whether to enable analytics
    #[serde(rename = "analytics", skip_serializing_if = "Option::is_none")]
    pub analytics: Option<bool>,
    /// Whether this server should be publicly discoverable
    #[serde(rename = "discoverable", skip_serializing_if = "Option::is_none")]
    pub discoverable: Option<bool>,
}

/// Optional fields on server object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerFields {
    Description,
    Categories,
    SystemMessages,
    Icon,
    Banner,
}