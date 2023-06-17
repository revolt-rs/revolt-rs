use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(tag = "channel_type")]
/// Representation of the available channels on Revolt
pub enum ChannelType { 
    #[serde(rename = "TextChannel")]
    /// Text channel belonging to a server
    TextChannel {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,
        /// Id of the server this channel belongs to
        server: String,
        /// Name of the channel
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Channel description
        description: Option<Option<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Icon attachment
        icon: Option<Option<Box<crate::models::ChannelOneOf2Icon>>>, 
        /// Id of the last message sent in this channel
        #[serde(rename = "last_message_id", skip_serializing_if = "Option::is_none")]
        last_message_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Permissions assigned based on role to this channel
        role_permissions: Option<HashMap<String, OverrideField>>,
        #[serde(skip_serializing_if = "Option::is_none")] 
        /// Whether this channel is marked as not safe for work
        nsfw: Option<bool>,
        
    },
    #[serde(rename = "VoiceChannel")]
    /// Voice channel belonging to a server
    VoiceChannel {
        /// Whether this channel is currently active
        active: bool,
        /// Whether this channel is currently active
        #[serde(rename = "voice_states")]
        voice_states: Vec<String>,
    },
    #[serde(rename = "DirectMessage")]
    /// Direct message channel between two users
    DirectMessage {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,
        /// Whether this DM channel is currently open on both sides
        active: bool,
        /// 2-tuple of user ids participating in DM
        recipients: Vec<String>,
        /// Id of the last message sent in this channel
        #[serde(rename = "last_message_id", default, skip_serializing_if = "Option::is_none")]
        last_message_id: Option<Option<String>>,
    },
    #[serde(rename = "Group")]
    /// Group channel between 1 or more participants
    Group {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,
        /// Whether this group channel is currently open on both sides
        active: bool,
        /// 2-tuple of user ids participating in group
        recipients: Vec<String>,
        /// Id of the last message sent in this channel
        #[serde(rename = "last_message_id", default, skip_serializing_if = "Option::is_none")]
        last_message_id: Option<Option<String>>,
    },
    #[serde(rename = "SavedMessages")]
    /// Personal "Saved Notes" channel which allows users to save messages
    SavedMessages {
        /// Unique Id
        #[serde(rename = "_id")]
        id: String,
        /// User Id
        user: String,
    }, 
}