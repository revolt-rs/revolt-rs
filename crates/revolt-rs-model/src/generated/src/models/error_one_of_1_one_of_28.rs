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
pub struct ErrorOneOf1OneOf28 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "max")]
    pub max: i32,
}

impl ErrorOneOf1OneOf28 {
    pub fn new(r#type: RHashType, max: i32) -> ErrorOneOf1OneOf28 {
        ErrorOneOf1OneOf28 {
            r#type,
            max,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "TooManyRoles")]
    TooManyRoles,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::TooManyRoles
    }
}

