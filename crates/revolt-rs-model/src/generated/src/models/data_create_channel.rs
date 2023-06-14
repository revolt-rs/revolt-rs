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
pub struct DataCreateChannel {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::models::ChannelType>,
    /// Channel name
    #[serde(rename = "name")]
    pub name: String,
    /// Channel description
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Whether this channel is age restricted
    #[serde(rename = "nsfw", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<Option<bool>>,
}

impl DataCreateChannel {
    pub fn new(name: String) -> DataCreateChannel {
        DataCreateChannel {
            r#type: None,
            name,
            description: None,
            nsfw: None,
        }
    }
}


