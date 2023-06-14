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
pub struct ErrorOneOf1OneOf19 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl ErrorOneOf1OneOf19 {
    pub fn new(r#type: RHashType) -> ErrorOneOf1OneOf19 {
        ErrorOneOf1OneOf19 {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "CannotRemoveYourself")]
    CannotRemoveYourself,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::CannotRemoveYourself
    }
}

