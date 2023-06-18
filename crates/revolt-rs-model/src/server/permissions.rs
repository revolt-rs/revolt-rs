use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PermissionsOverwrite {
    /// Allow bit flags
    #[serde(rename = "allow")]
    pub allow: i32,
    /// Disallow bit flags
    #[serde(rename = "deny")]
    pub deny: i32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i64)]
pub enum Permission {
    ManageChannels = 0b1 << 0,
    ManageServer = 0b1 << 1,
    ManagePermissions = 0b1 << 2,
    ManageRoles = 0b1 << 3,
    ManageCustomisation = 0b1 << 4,
    KickMembers = 0b1 << 6,
    BanMembers = 0b1 << 7,
    TimeoutMembers = 0b1 << 8,
    AssignRoles = 0b1 << 9,
    ChangeNickname = 0b1 << 10,
    ManageNicknames = 0b1 << 11,
    ChangeAvatar = 0b1 << 12,
    ManageAvatars = 0b1 << 13,
    ViewChannels = 0b1 << 20,
    SendMessages = 0b1 << 22,
    ManageMessages = 0b1 << 23,
    ManageWebhooks = 0b1 << 24,
    CreateInvites = 0b1 << 25,
    SendEmbeds = 0b1 << 26,
    UploadFiles = 0b1 << 27,
    Masquerade = 0b1 << 28,
    AddReactions = 0b1 << 29,
    VoiceConnect = 0b1 << 30,
    VoiceSpeak = 0b1 << 31,
    VoiceVideo = 0b1 << 32,
    VoiceMuteMembers = 0b1 << 33,
    VoiceDeafenMembers = 0b1 << 34,
    VoiceMoveMembers = 0b1 << 35,
}
