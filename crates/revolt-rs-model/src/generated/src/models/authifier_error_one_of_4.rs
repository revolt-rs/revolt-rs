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
pub struct AuthifierErrorOneOf4 {
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl AuthifierErrorOneOf4 {
    pub fn new(r#type: RHashType) -> AuthifierErrorOneOf4 {
        AuthifierErrorOneOf4 {
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "CaptchaFailed")]
    CaptchaFailed,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::CaptchaFailed
    }
}

