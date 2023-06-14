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
pub struct CaptchaFeature {
    /// Whether captcha is enabled
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Client key used for solving captcha
    #[serde(rename = "key")]
    pub key: String,
}

impl CaptchaFeature {
    pub fn new(enabled: bool, key: String) -> CaptchaFeature {
        CaptchaFeature {
            enabled,
            key,
        }
    }
}

