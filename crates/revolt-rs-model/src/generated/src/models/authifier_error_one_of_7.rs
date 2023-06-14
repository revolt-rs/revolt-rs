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
pub struct AuthifierErrorOneOf7 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl AuthifierErrorOneOf7 {
    pub fn new(r#type: RHashType) -> AuthifierErrorOneOf7 {
        AuthifierErrorOneOf7 {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "EmailFailed")]
    EmailFailed,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::EmailFailed
    }
}

