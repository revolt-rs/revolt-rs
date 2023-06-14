/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// FetchBotResponseBot : Bot object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FetchBotResponseBot {
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
    #[serde(rename = "interactions_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub interactions_url: Option<Option<String>>,
    /// URL for terms of service
    #[serde(rename = "terms_of_service_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<Option<String>>,
    /// URL for privacy policy
    #[serde(rename = "privacy_policy_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<Option<String>>,
    /// Enum of bot flags
    #[serde(rename = "flags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Option<i32>>,
}

impl FetchBotResponseBot {
    /// Bot object
    pub fn new(_id: String, owner: String, token: String, public: bool) -> FetchBotResponseBot {
        FetchBotResponseBot {
            _id,
            owner,
            token,
            public,
            analytics: None,
            discoverable: None,
            interactions_url: None,
            terms_of_service_url: None,
            privacy_policy_url: None,
            flags: None,
        }
    }
}


