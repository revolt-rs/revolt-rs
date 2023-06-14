/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// User : Representiation of a User on Revolt.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct User {
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Username
    #[serde(rename = "username")]
    pub username: String,
    /// Discriminator
    #[serde(rename = "discriminator")]
    pub discriminator: String,
    /// Display name
    #[serde(rename = "display_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<Option<String>>,
    #[serde(rename = "avatar", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<Box<crate::models::UserAvatar>>>,
    /// Relationships with other users
    #[serde(rename = "relations", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub relations: Option<Option<Vec<crate::models::Relationship>>>,
    /// Bitfield of user badges
    #[serde(rename = "badges", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub badges: Option<Option<i32>>,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<Box<crate::models::UserStatus>>>,
    #[serde(rename = "profile", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub profile: Option<Option<Box<crate::models::UserProfile>>>,
    /// Enum of user flags
    #[serde(rename = "flags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Option<i32>>,
    /// Whether this user is privileged
    #[serde(rename = "privileged", skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "bot", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bot: Option<Option<Box<crate::models::UserBot>>>,
    #[serde(rename = "relationship", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Option<crate::models::RelationshipStatus>>,
    /// Whether this user is currently online
    #[serde(rename = "online", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub online: Option<Option<bool>>,
}

impl User {
    /// Representiation of a User on Revolt.
    pub fn new(_id: String, username: String, discriminator: String) -> User {
        User {
            _id,
            username,
            discriminator,
            display_name: None,
            avatar: None,
            relations: None,
            badges: None,
            status: None,
            profile: None,
            flags: None,
            privileged: None,
            bot: None,
            relationship: None,
            online: None,
        }
    }
}

