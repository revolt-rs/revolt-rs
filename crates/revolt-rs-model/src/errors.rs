#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum ErrorType{
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
