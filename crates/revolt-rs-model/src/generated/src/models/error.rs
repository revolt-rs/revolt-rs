/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Error : Possible API Errors



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "max")]
    pub max: i32,
    #[serde(rename = "permission")]
    pub permission: crate::models::UserPermission,
    #[serde(rename = "operation")]
    pub operation: String,
    #[serde(rename = "collection")]
    pub collection: String,
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "with")]
    pub with: String,
}

impl Error {
    /// Possible API Errors
    pub fn new(r#type: RHashType, max: i32, permission: crate::models::UserPermission, operation: String, collection: String, error: String, with: String) -> Error {
        Error {
            r#type,
            max,
            permission,
            operation,
            collection,
            error,
            with,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "LabelMe")]
    LabelMe,
    #[serde(rename = "VosoUnavailable")]
    VosoUnavailable,
    #[serde(rename = "AlreadyOnboarded")]
    AlreadyOnboarded,
    #[serde(rename = "UsernameTaken")]
    UsernameTaken,
    #[serde(rename = "InvalidUsername")]
    InvalidUsername,
    #[serde(rename = "UnknownUser")]
    UnknownUser,
    #[serde(rename = "AlreadyFriends")]
    AlreadyFriends,
    #[serde(rename = "AlreadySentRequest")]
    AlreadySentRequest,
    #[serde(rename = "Blocked")]
    Blocked,
    #[serde(rename = "BlockedByOther")]
    BlockedByOther,
    #[serde(rename = "NotFriends")]
    NotFriends,
    #[serde(rename = "UnknownChannel")]
    UnknownChannel,
    #[serde(rename = "UnknownAttachment")]
    UnknownAttachment,
    #[serde(rename = "UnknownMessage")]
    UnknownMessage,
    #[serde(rename = "CannotEditMessage")]
    CannotEditMessage,
    #[serde(rename = "CannotJoinCall")]
    CannotJoinCall,
    #[serde(rename = "TooManyAttachments")]
    TooManyAttachments,
    #[serde(rename = "TooManyReplies")]
    TooManyReplies,
    #[serde(rename = "TooManyChannels")]
    TooManyChannels,
    #[serde(rename = "TooManyEmbeds")]
    TooManyEmbeds,
    #[serde(rename = "EmptyMessage")]
    EmptyMessage,
    #[serde(rename = "PayloadTooLarge")]
    PayloadTooLarge,
    #[serde(rename = "CannotRemoveYourself")]
    CannotRemoveYourself,
    #[serde(rename = "GroupTooLarge")]
    GroupTooLarge,
    #[serde(rename = "AlreadyInGroup")]
    AlreadyInGroup,
    #[serde(rename = "NotInGroup")]
    NotInGroup,
    #[serde(rename = "UnknownServer")]
    UnknownServer,
    #[serde(rename = "InvalidRole")]
    InvalidRole,
    #[serde(rename = "Banned")]
    Banned,
    #[serde(rename = "TooManyServers")]
    TooManyServers,
    #[serde(rename = "TooManyEmoji")]
    TooManyEmoji,
    #[serde(rename = "TooManyRoles")]
    TooManyRoles,
    #[serde(rename = "ReachedMaximumBots")]
    ReachedMaximumBots,
    #[serde(rename = "IsBot")]
    IsBot,
    #[serde(rename = "BotIsPrivate")]
    BotIsPrivate,
    #[serde(rename = "CannotReportYourself")]
    CannotReportYourself,
    #[serde(rename = "MissingPermission")]
    MissingPermission,
    #[serde(rename = "MissingUserPermission")]
    MissingUserPermission,
    #[serde(rename = "NotElevated")]
    NotElevated,
    #[serde(rename = "NotPrivileged")]
    NotPrivileged,
    #[serde(rename = "CannotGiveMissingPermissions")]
    CannotGiveMissingPermissions,
    #[serde(rename = "NotOwner")]
    NotOwner,
    #[serde(rename = "DatabaseError")]
    DatabaseError,
    #[serde(rename = "InternalError")]
    InternalError,
    #[serde(rename = "InvalidOperation")]
    InvalidOperation,
    #[serde(rename = "InvalidCredentials")]
    InvalidCredentials,
    #[serde(rename = "InvalidProperty")]
    InvalidProperty,
    #[serde(rename = "InvalidSession")]
    InvalidSession,
    #[serde(rename = "DuplicateNonce")]
    DuplicateNonce,
    #[serde(rename = "NotFound")]
    NotFound,
    #[serde(rename = "NoEffect")]
    NoEffect,
    #[serde(rename = "FailedValidation")]
    FailedValidation,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::LabelMe
    }
}

