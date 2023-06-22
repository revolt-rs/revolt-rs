use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::id::{Id, ServerMarker, UserMarker};

use super::{ChannelIcon, OverrideField};

fn is_false(b: &bool) -> bool {
    *b == false
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// Text channel belonging to a server
pub struct TextChannel {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Id of the server this channel belongs to
    pub server: Id<ServerMarker>,
    /// Name of the channel
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Channel description
    pub description: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Icon attachment
    pub icon: Option<Option<Box<ChannelIcon>>>,
    /// Id of the last message sent in this channel
    #[serde(rename = "last_message_id", skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Permissions assigned based on role to this channel
    pub role_permissions: Option<HashMap<String, OverrideField>>,
    #[serde(default)]
    /// Whether this channel is marked as not safe for work
    pub nsfw: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// Voice channel belonging to a server
pub struct VoiceChannel {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Whether this channel is currently active
    pub active: bool,
    /// Server [`Id<ServerMarker>`] this channel belongs to
    pub server: Id<ServerMarker>,
    /// Channel name
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Channel description
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Icon attachment
    pub icon: Option<ChannelIcon>,
    #[serde(default)]
    /// Default channel permissions
    pub default_permissions: OverrideField,
    #[serde(default)]
    /// Permissions based on role
    pub role_permissions: HashMap<String, OverrideField>,
    #[serde(default)]
    /// Whether this channel is marked as not safe for work
    pub nsfw: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Direct message channel between two users
pub struct DirectMessage {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Whether this DM channel is currently open on both sides
    pub active: bool,
    /// 2-tuple of user ids participating in DM
    pub recipients: Vec<String>,
    /// Id of the last message sent in this channel
    #[serde(
        rename = "last_message_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_message_id: Option<Option<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Group channel between 1 or more participants
pub struct Group {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// List of user ids participating in this group
    pub recipients: Vec<String>,
    /// Name of the group
    pub name: String,
    /// Owner of the group
    pub owner: String,
    /// Description of the group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ChannelIcon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<u64>,
    #[serde(default)]
    pub nsfw: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Personal "Saved Notes" channel which allows users to save messages
pub struct SavedMessages {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// User Id
    pub user: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "channel_type")]

/// Representation of the available channels on Revolt
pub enum ChannelType {
    SavedMessages(SavedMessages),
    DirectMessage(DirectMessage),
    Group(Group),
    TextChannel(TextChannel),
    VoiceChannel(VoiceChannel),
}

/// Optional fields on channel object
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelFields {
    Description,
    Icon,
    DefaultPermissions,
}

impl DirectMessage {
    /// Returns the other user's Id
    pub fn get_recipient(&self, user: Id<UserMarker>) -> Option<&str> {
        self.recipients
            .iter()
            .find(|&x| x != user.get())
            .map(|x| x.as_str())
    }
}

impl ChannelType {
    pub fn server(&self) -> Option<&str> {
        match self {
            ChannelType::TextChannel(x) => Some(x.server.get()),
            ChannelType::VoiceChannel(x) => Some(x.server.get()),
            _ => None,
        }
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            ChannelType::TextChannel(x) => Some(&x.name),
            ChannelType::VoiceChannel(x) => Some(&x.name),
            ChannelType::Group(x) => Some(&x.name),
            _ => None,
        }
    }

    pub fn id(&self) -> Option<&String> {
        match self {
            ChannelType::TextChannel(x) => Some(&x.id),
            ChannelType::VoiceChannel(x) => Some(&x.id),
            ChannelType::Group(x) => Some(&x.id),
            ChannelType::DirectMessage(x) => Some(&x.id),
            ChannelType::SavedMessages(x) => Some(&x.id),
        }
    }

    pub fn description(&self) -> Option<&String> {
        match self {
            ChannelType::TextChannel(x) => x.description.as_ref()?.as_ref(),
            ChannelType::VoiceChannel(x) => x.description.as_ref(),
            ChannelType::Group(x) => x.description.as_ref(),
            _ => None,
        }
    }

    pub fn icon(&self) -> Option<&ChannelIcon> {
        match self {
            ChannelType::TextChannel(x) => x.icon.as_ref()?.as_deref(),
            ChannelType::VoiceChannel(x) => x.icon.as_ref(),
            ChannelType::Group(x) => x.icon.as_ref(),
            _ => None,
        }
    }

    pub fn lastest_message(&self) -> Option<&String> {
        match self {
            ChannelType::TextChannel(x) => x.last_message_id.as_ref(),
            ChannelType::Group(x) => x.last_message_id.as_ref(),
            ChannelType::DirectMessage(x) => x.last_message_id.as_ref()?.as_ref(),
            _ => None,
        }
    } 
}
