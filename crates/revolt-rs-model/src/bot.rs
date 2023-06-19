use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Bot {
    /// Bot Id  This equals the associated bot user's id.
    #[serde(rename = "_id")]
    pub _id: String,
    /// User Id of the bot owner
    #[serde(rename = "owner")]
    pub owner: String,
    /// Token used to authenticate requests for this bot
    #[serde(rename = "token")]
    pub token: String,
    /// Whether the bot is public (may be invited by anyone)
    #[serde(rename = "public")]
    pub public: bool,
    /// Whether to enable analytics
    #[serde(rename = "analytics", skip_serializing_if = "Option::is_none")]
    pub analytics: Option<bool>,
    /// Whether this bot should be publicly discoverable
    #[serde(rename = "discoverable", skip_serializing_if = "Option::is_none")]
    pub discoverable: Option<bool>,
    /// Reserved; URL for handling interactions
    #[serde(rename = "interactions_url", default, skip_serializing_if = "Option::is_none")]
    pub interactions_url: Option<Option<String>>,
    /// URL for terms of service
    #[serde(rename = "terms_of_service_url", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<Option<String>>,
    /// URL for privacy policy
    #[serde(rename = "privacy_policy_url", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<Option<String>>,
    /// Enum of bot flags
    #[serde(rename = "flags", default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<Option<i32>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PublicBot {
    /// Bot Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Bot Username
    #[serde(rename = "username")]
    pub username: String,
    /// Profile Avatar
    #[serde(rename = "avatar")]
    pub avatar: String,
    /// Profile Description
    #[serde(rename = "description")]
    pub description: String,
}
