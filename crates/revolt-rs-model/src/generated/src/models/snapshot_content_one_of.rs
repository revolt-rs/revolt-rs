/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// SnapshotContentOneOf : Representation of a Message on Revolt



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SnapshotContentOneOf {
    #[serde(rename = "_type")]
    pub _type: Type,
    /// Context before the message
    #[serde(rename = "_prior_context", skip_serializing_if = "Option::is_none")]
    pub _prior_context: Option<Vec<crate::models::Message>>,
    /// Context after the message
    #[serde(rename = "_leading_context", skip_serializing_if = "Option::is_none")]
    pub _leading_context: Option<Vec<crate::models::Message>>,
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Unique value generated by client sending this message
    #[serde(rename = "nonce", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Option<String>>,
    /// Id of the channel this message was sent in
    #[serde(rename = "channel")]
    pub channel: String,
    /// Id of the user or webhook that sent this message
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "webhook", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Option<Box<crate::models::MessageWebhook>>>,
    /// Message content
    #[serde(rename = "content", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
    #[serde(rename = "system", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub system: Option<Option<Box<crate::models::MessageSystem>>>,
    /// Array of attachments
    #[serde(rename = "attachments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Option<Vec<crate::models::File>>>,
    /// ISO8601 formatted timestamp
    #[serde(rename = "edited", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub edited: Option<Option<String>>,
    /// Attached embeds to this message
    #[serde(rename = "embeds", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Option<Vec<crate::models::Embed>>>,
    /// Array of user ids mentioned in this message
    #[serde(rename = "mentions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Option<Vec<String>>>,
    /// Array of message ids this message is replying to
    #[serde(rename = "replies", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub replies: Option<Option<Vec<String>>>,
    /// Hashmap of emoji IDs to array of user IDs
    #[serde(rename = "reactions", skip_serializing_if = "Option::is_none")]
    pub reactions: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "interactions", skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Box<crate::models::MessageInteractions>>,
    #[serde(rename = "masquerade", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub masquerade: Option<Option<Box<crate::models::MessageMasquerade>>>,
}

impl SnapshotContentOneOf {
    /// Representation of a Message on Revolt
    pub fn new(_type: Type, _id: String, channel: String, author: String) -> SnapshotContentOneOf {
        SnapshotContentOneOf {
            _type,
            _prior_context: None,
            _leading_context: None,
            _id,
            nonce: None,
            channel,
            author,
            webhook: None,
            content: None,
            system: None,
            attachments: None,
            edited: None,
            embeds: None,
            mentions: None,
            replies: None,
            reactions: None,
            interactions: None,
            masquerade: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Message")]
    Message,
}

impl Default for Type {
    fn default() -> Type {
        Self::Message
    }
}
