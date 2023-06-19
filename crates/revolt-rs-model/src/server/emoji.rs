use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EmojiParent {
    Server { 
        id: String 
    },
    Detached,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Emoji {
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "parent")]
    pub parent: Box<EmojiParent>,
    /// Uploader user id
    #[serde(rename = "creator_id")]
    pub creator_id: String,
    /// Emoji name
    #[serde(rename = "name")]
    pub name: String,
    /// Whether the emoji is animated
    #[serde(rename = "animated", skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
    /// Whether the emoji is marked as nsfw
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}
