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
pub struct ErrorOneOf1OneOf27 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "max")]
    pub max: i32,
}

impl ErrorOneOf1OneOf27 {
    pub fn new(r#type: RHashType, max: i32) -> ErrorOneOf1OneOf27 {
        ErrorOneOf1OneOf27 {
            r#type,
            max,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "TooManyEmoji")]
    TooManyEmoji,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::TooManyEmoji
    }
}

