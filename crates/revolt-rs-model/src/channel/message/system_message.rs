use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SystemMessageType {
    #[serde(rename = "text")]
    Text { content: String },
    #[serde(rename = "user_added")]
    UserAdded { id: String, by: String },
    #[serde(rename = "user_remove")]
    UserRemove { id: String, by: String },
    #[serde(rename = "user_joined")]
    UserJoined { id: String },
    #[serde(rename = "user_left")]
    UserLeft { id: String },
    #[serde(rename = "user_kicked")]
    UserKicked { id: String },
    #[serde(rename = "user_banned")]
    UserBanned { id: String },
    #[serde(rename = "channel_renamed")]
    ChannelRenamed { name: String, by: String },
    #[serde(rename = "channel_description_changed")]
    ChannelDescriptionChanged { by: String },
    #[serde(rename = "channel_icon_changed")]
    ChannelIconChanged { by: String },
    #[serde(rename = "channel_ownership_changed")]
    ChannelOwnershipChanged { from: String, to: String },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemMessage {
    #[serde(rename = "type")]
    pub system_message_type: SystemMessageType,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "by")]
    pub by: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "to")]
    pub to: String,
}
