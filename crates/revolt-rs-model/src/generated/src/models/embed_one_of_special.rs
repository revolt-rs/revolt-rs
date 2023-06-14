/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// EmbedOneOfSpecial : Remote content



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EmbedOneOfSpecial {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "timestamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Option<String>>,
    #[serde(rename = "content_type")]
    pub content_type: crate::models::BandcampType,
}

impl EmbedOneOfSpecial {
    /// Remote content
    pub fn new(r#type: RHashType, id: String, content_type: crate::models::BandcampType) -> EmbedOneOfSpecial {
        EmbedOneOfSpecial {
            r#type,
            id,
            timestamp: None,
            content_type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Streamable")]
    Streamable,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Streamable
    }
}

