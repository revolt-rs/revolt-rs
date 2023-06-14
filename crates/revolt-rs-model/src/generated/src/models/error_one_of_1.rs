/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ErrorOneOf1 : Core crate error



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ErrorOneOf1 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// Where this error occurred
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "max")]
    pub max: i32,
    #[serde(rename = "permission")]
    pub permission: String,
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "error")]
    pub error: String,
}

impl ErrorOneOf1 {
    /// Core crate error
    pub fn new(r#type: RHashType, location: String, max: i32, permission: String, operation: String, collection: String, error: String) -> ErrorOneOf1 {
        ErrorOneOf1 {
            r#type,
            location,
            max,
            permission,
            operation,
            collection,
            error,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Core")]
    Core,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Core
    }
}

