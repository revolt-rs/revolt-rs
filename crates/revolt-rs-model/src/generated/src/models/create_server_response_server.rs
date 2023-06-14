/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// CreateServerResponseServer : Server object



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateServerResponseServer {
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
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Channels within this server
    #[serde(rename = "channels")]
    pub channels: Vec<String>,
    /// Categories for this server
    #[serde(rename = "categories", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Option<Vec<crate::models::Category>>>,
    #[serde(rename = "system_messages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub system_messages: Option<Option<Box<crate::models::ServerSystemMessages>>>,
    /// Roles for this server
    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<::std::collections::HashMap<String, crate::models::Role>>,
    /// Default set of server and channel permissions
    #[serde(rename = "default_permissions")]
    pub default_permissions: i64,
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<Box<crate::models::ServerIcon>>>,
    #[serde(rename = "banner", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub banner: Option<Option<Box<crate::models::ServerBanner>>>,
    /// Bitfield of server flags
    #[serde(rename = "flags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
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

impl CreateServerResponseServer {
    /// Server object
    pub fn new(_id: String, owner: String, name: String, channels: Vec<String>, default_permissions: i64) -> CreateServerResponseServer {
        CreateServerResponseServer {
            _id,
            owner,
            name,
            description: None,
            channels,
            categories: None,
            system_messages: None,
            roles: None,
            default_permissions,
            icon: None,
            banner: None,
            flags: None,
            nsfw: None,
            analytics: None,
            discoverable: None,
        }
    }
}


