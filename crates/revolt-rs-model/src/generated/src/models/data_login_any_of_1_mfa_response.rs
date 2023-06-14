/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// DataLoginAnyOf1MfaResponse : Valid MFA response  This will take precedence over the `password` field where applicable



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataLoginAnyOf1MfaResponse {
    #[serde(rename = "password")]
    pub password: String,
    #[serde(rename = "recovery_code")]
    pub recovery_code: String,
    #[serde(rename = "totp_code")]
    pub totp_code: String,
}

impl DataLoginAnyOf1MfaResponse {
    /// Valid MFA response  This will take precedence over the `password` field where applicable
    pub fn new(password: String, recovery_code: String, totp_code: String) -> DataLoginAnyOf1MfaResponse {
        DataLoginAnyOf1MfaResponse {
            password,
            recovery_code,
            totp_code,
        }
    }
}


