/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// SpecialOneOf5 : Spotify track



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SpecialOneOf5 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "id")]
    pub id: String,
}

impl SpecialOneOf5 {
    /// Spotify track
    pub fn new(r#type: RHashType, content_type: String, id: String) -> SpecialOneOf5 {
        SpecialOneOf5 {
            r#type,
            content_type,
            id,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Spotify")]
    Spotify,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Spotify
    }
}

