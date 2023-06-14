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
pub struct ErrorOneOf3 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "permission")]
    pub permission: crate::models::Permission,
}

impl ErrorOneOf3 {
    pub fn new(r#type: RHashType, permission: crate::models::Permission) -> ErrorOneOf3 {
        ErrorOneOf3 {
            r#type,
            permission,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "MissingPermission")]
    MissingPermission,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::MissingPermission
    }
}
