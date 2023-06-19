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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SinglePermissionOverwrite {
    /// Allow bit flags
    #[serde(rename = "allow")]
    pub a: i32,
    /// Disallow bit flags
    #[serde(rename = "deny")]
    pub d: i32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i64)]
pub enum Permission {
    /// Allows the user to create new channels.
    ManageChannels = 0b1 << 0,
    /// Allows the user to manage the server.
    ManageServer = 0b1 << 1,
    /// Allows the user to manage permissions.
    ManagePermissions = 0b1 << 2,
    /// Allows the user to manage roles.
    ManageRoles = 0b1 << 3,
    /// Allows the user to manage the server.
    ManageCustomisation = 0b1 << 4,
    /// Allows the user to kick members.
    KickMembers = 0b1 << 6,
    /// Allows the user to ban members.
    BanMembers = 0b1 << 7,
    /// Allows the user to mute members.
    TimeoutMembers = 0b1 << 8,
    /// Allows the user to assign roles.
    AssignRoles = 0b1 << 9,
    /// Allows the user to change their nickname.
    ChangeNickname = 0b1 << 10,
    /// Allows the user to manage nicknames.
    ManageNicknames = 0b1 << 11,
    /// Allows the user to change their avatar.
    ChangeAvatar = 0b1 << 12,
    /// Allows the user to manage avatars.
    ManageAvatars = 0b1 << 13,
    /// Allows the user to view a channel.
    ViewChannels = 0b1 << 20,
    /// Allows the user to send messages.
    SendMessages = 0b1 << 22,
    /// Allows the user to manage messages.
    ManageMessages = 0b1 << 23,
    /// Allows the user to manage webhooks.
    ManageWebhooks = 0b1 << 24,
    /// Allows the user to create invites.
    CreateInvites = 0b1 << 25,
    /// Allows the user to send embeds.
    SendEmbeds = 0b1 << 26,
    /// Allows the user to upload files.
    UploadFiles = 0b1 << 27,
    /// Masquerade messages using custom nickname and avatar
    Masquerade = 0b1 << 28,
    /// Allows the user to add reactions.
    AddReactions = 0b1 << 29,
    /// Allows the user to use voice connect.
    VoiceConnect = 0b1 << 30,
    /// Allows the user to use voice speak.
    VoiceSpeak = 0b1 << 31,
    /// Allows the user to use voice video.
    VoiceVideo = 0b1 << 32,
    /// Allows the user to mute members.
    VoiceMuteMembers = 0b1 << 33,
    /// Allows the user to deafen members.
    VoiceDeafenMembers = 0b1 << 34,
    /// Allows the user to move members.
    VoiceMoveMembers = 0b1 << 35,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i16)]
pub enum UserPermission {
    Access = 0b1 << 0,
    ViewProfile = 0b1 << 1,
    SendMessage = 0b1 << 2,
    Invite = 0b1 << 3,
}
