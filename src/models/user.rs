use super::attachment::File;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct UserProfile {
    /// The user's content on their profile. (Bio)
    pub content: Option<String>,
    /// The user's wallpaper.
    pub wallpaper: Option<File>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
#[non_exhaustive]
pub enum Presence {
    /// User is online.
    Online,
    /// User is not currently available.
    Idle,
    /// User is focusing / will only receive mentions.
    Focus,
    /// User is busy / will not receive any notifications.
    Busy,
    /// User appears to be offline.
    Invisible,
}

impl Presence {
    #[must_use]
    pub fn name(&self) -> &str {
        match *self {
            Presence::Online => "Online",
            Presence::Idle => "Idle",
            Presence::Focus => "Focus",
            Presence::Busy => "Busy",
            Presence::Invisible => "Invisible",
        }
    }
}

impl Default for Presence {
    fn default() -> Presence {
        Presence::Online
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RelationshipStatus {
    None,
    User,
    Friend,
    Outgoing,
    Incoming,
    Blocked,
    BlockedOther,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relationship {
    #[serde(rename = "_id")]
    pub id: String,
    pub status: RelationshipStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate, Default)]
pub struct UserStatus {
    /// Custom status text
    #[validate(length(min = 1, max = 128))]
    pub text: Option<String>,
    pub presence: Option<Presence>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum Badges {
    /// Revolt Developer
    Developer = 1,
    /// Helped translate Revolt
    Translator = 2,
    /// Monetarily supported Revolt
    Supporter = 4,
    /// Responsibly disclosed a security issue
    ResponsibleDisclosure = 8,
    /// Revolt Founder
    Founder = 16,
    /// Platform moderator
    PlatformModeration = 32,
    /// Active monetary supporter
    ActiveSupporter = 64,
    /// 🦊🦝
    Paw = 128,
    /// Joined as one of the first 1000 users in 2021
    EarlyAdopter = 256,
    /// Amogus
    ReservedRelevantJokeBadge1 = 512,
    /// Low resolution troll face
    ReservedRelevantJokeBadge2 = 1024,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum Flags {
    /// User has been suspended from the platform
    Suspended = 1,
    /// User has deleted their account
    Deleted = 2,
    /// User was banned off the platform
    Banned = 4,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotInformation {
    /// Id of the owner of this bot
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// Unique Id
    #[serde(rename = "_id")]
    pub id: String,
    /// Username
    pub username: String,
    /// Discriminator
    pub discriminator: u16,
    /// User's display name
    pub display_name: String,
    /// Avatar attachment
    pub avatar: Option<File>,
    /// Relationships with other users
    pub relations: Option<Vec<Relationship>>,
    /// Bitfield of user badges
    pub badges: Option<i32>,
    /// User's current status
    pub status: Option<UserStatus>,
    /// User's profile page
    pub profile: Option<UserProfile>,
    /// Enum of user flags
    pub flags: Option<i32>,
    /// Whether this user is privileged
    pub privileged: bool,
    /// Bot information
    pub bot: Option<BotInformation>,
    /// Current session user's relationship with this user
    pub relationship: Option<RelationshipStatus>,
    /// Whether this user is currently online
    pub online: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldsUser {
    Avatar,
    StatusText,
    StatusPresence,
    ProfileContent,
    ProfileBackground,
}

pub enum UserHint {
    /// Could be either a user or a bot
    Any,
    /// Only match bots
    Bot,
    /// Only match users
    User,
}

impl User {
    #[inline]
    #[must_use]
    pub fn tag(&self) -> String {
        tag(&self.username, self.discriminator)
    }
    #[inline]
    #[must_use]
    pub fn discriminator(&self) -> u16 {
        discriminator(self.discriminator)
    } 
}

impl Default for User {
    /// Initializes a [`User`] with default values. Setting the following:
    /// - **id** to `UserId(210)`
    /// - **avatar** to `Some("abc")`
    /// - **bot** to `true`.
    /// - **discriminator** to `1337`.
    /// - **name** to `"test"`.
    /// - **public_flags** to [`None`].
    fn default() -> Self {
        User {
            id: "01EZMT96C3YJ7T2NN996T8VXJE".to_string(),
            username: "kyle".to_string(),
            avatar: None,
            discriminator: 1337,
            display_name: "Kyle".to_string(),
            badges: None,
            privileged: true,
            relations: None,
            status: None,
            profile: None,
            flags: None,
            bot: None,
            relationship: None,
            online: Some(true),
        }
    }
}

fn tag(name: &str, discriminator: u16) -> String {
    return format!("{}#{}", name, discriminator);
}

fn discriminator(discriminator: u16) -> u16 {
    return discriminator;
}
