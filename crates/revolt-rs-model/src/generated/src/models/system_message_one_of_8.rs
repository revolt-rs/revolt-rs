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
pub struct SystemMessageOneOf8 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "by")]
    pub by: String,
}

impl SystemMessageOneOf8 {
    pub fn new(r#type: RHashType, by: String) -> SystemMessageOneOf8 {
        SystemMessageOneOf8 {
            r#type,
            by,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "channel_description_changed")]
    ChannelDescriptionChanged,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::ChannelDescriptionChanged
    }
}
