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
pub struct AuthifierErrorOneOf {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "with")]
    pub with: String,
}

impl AuthifierErrorOneOf {
    pub fn new(r#type: RHashType, with: String) -> AuthifierErrorOneOf {
        AuthifierErrorOneOf {
            r#type,
            with,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "IncorrectData")]
    IncorrectData,
    #[serde(rename = "DatabaseError")]
    DatabaseError,
    #[serde(rename = "InternalError")]
    InternalError,
    #[serde(rename = "OperationFailed")]
    OperationFailed,
    #[serde(rename = "RenderFail")]
    RenderFail,
    #[serde(rename = "MissingHeaders")]
    MissingHeaders,
    #[serde(rename = "CaptchaFailed")]
    CaptchaFailed,
    #[serde(rename = "BlockedByShield")]
    BlockedByShield,
    #[serde(rename = "InvalidSession")]
    InvalidSession,
    #[serde(rename = "UnverifiedAccount")]
    UnverifiedAccount,
    #[serde(rename = "UnknownUser")]
    UnknownUser,
    #[serde(rename = "EmailFailed")]
    EmailFailed,
    #[serde(rename = "InvalidToken")]
    InvalidToken,
    #[serde(rename = "MissingInvite")]
    MissingInvite,
    #[serde(rename = "InvalidInvite")]
    InvalidInvite,
    #[serde(rename = "InvalidCredentials")]
    InvalidCredentials,
    #[serde(rename = "CompromisedPassword")]
    CompromisedPassword,
    #[serde(rename = "ShortPassword")]
    ShortPassword,
    #[serde(rename = "Blacklisted")]
    Blacklisted,
    #[serde(rename = "LockedOut")]
    LockedOut,
    #[serde(rename = "TotpAlreadyEnabled")]
    TotpAlreadyEnabled,
    #[serde(rename = "DisallowedMFAMethod")]
    DisallowedMfaMethod,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::IncorrectData
    }
}
