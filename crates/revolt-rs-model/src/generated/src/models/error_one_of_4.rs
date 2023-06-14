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
pub struct ErrorOneOf4 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "permission")]
    pub permission: crate::models::UserPermission,
}

impl ErrorOneOf4 {
    pub fn new(r#type: RHashType, permission: crate::models::UserPermission) -> ErrorOneOf4 {
        ErrorOneOf4 {
            r#type,
            permission,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "MissingUserPermission")]
    MissingUserPermission,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::MissingUserPermission
    }
}
