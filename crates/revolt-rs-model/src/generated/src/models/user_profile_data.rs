/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserProfileData {
    /// Text to set as user profile description
    #[serde(rename = "content", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content: Option<Option<String>>,
    /// Attachment Id for background
    #[serde(rename = "background", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub background: Option<Option<String>>,
}

impl UserProfileData {
    pub fn new() -> UserProfileData {
        UserProfileData {
            content: None,
            background: None,
        }
    }
}

