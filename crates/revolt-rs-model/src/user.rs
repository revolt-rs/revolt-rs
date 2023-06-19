use crate::id::{Id, UserMarker};

use super::attachment::File;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[non_exhaustive]
#[allow(clippy::module_name_repetitions)]
pub struct UserProfile {
    /// The user's content on their profile. (Bio)
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub content: Option<String>,
    /// The user's wallpaper.
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub wallpaper: Option<File>,
}

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, Default,
)]
#[non_exhaustive]
pub enum Presence {
    /// User is online.
    #[default]
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

impl std::fmt::Display for Presence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match *self {
            Self::Online => "Online",
            Self::Idle => "Idle",
            Self::Focus => "Focus",
            Self::Busy => "Busy",
            Self::Invisible => "Invisible",
        };
        f.write_str(formatted)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
#[allow(clippy::module_name_repetitions)]
pub struct UserStatus {
    /// Custom status text
    #[validate(length(min = 1, max = 128))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<Presence>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum Badges {
    /// Revolt Developer
    Developer = 0b1 << 1,
    /// Helped translate Revolt
    Translator = 0b1 << 2,
    /// Monetarily supported Revolt
    Supporter = 0b1 << 3,
    /// Responsibly disclosed a security issue
    ResponsibleDisclosure = 0b1 << 4,
    /// Revolt Founder
    Founder = 0b1 << 5,
    /// Platform moderator
    PlatformModeration = 0b1 << 6,
    /// Active monetary supporter
    ActiveSupporter = 0b1 << 7,
    /// 🦊🦝
    Paw = 0b1 << 8,
    /// Joined as one of the first 1000 users in 2021
    EarlyAdopter = 0b1 << 9,
    /// Amogus
    ReservedRelevantJokeBadge1 = 0b1 << 10,
    /// Low resolution troll face
    ReservedRelevantJokeBadge2 = 0b1 << 11,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(i32)]
pub enum Flags {
    /// User has been suspended from the platform
    Suspended = 0b1 << 1,
    /// User has deleted their account
    Deleted = 0b1 << 2,
    /// User was banned off the platform
    Banned = 0b1 << 3,
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
    pub id: Id<UserMarker>,
    /// Username
    pub username: String,
    /// Discriminator
    pub discriminator: u16,
    /// User's display name
    pub display_name: String,
    /// Avatar attachment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<File>,
    /// Relationships with other users
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub relations: Option<Vec<Relationship>>,
    /// Bitfield of user badges
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badges: Option<i32>,
    /// User's current status
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub status: Option<UserStatus>,
    /// User's profile page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<UserProfile>,
    /// Enum of user flags
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub flags: Option<i32>,
    /// Whether this user is privileged
    pub privileged: bool,
    /// Bot information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<BotInformation>,
    /// Current session user's relationship with this user
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub relationship: Option<RelationshipStatus>,
    /// Whether this user is currently online
    #[serde(skip_serializing_if = "Option::is_none")] 
    pub online: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub enum UserFields {
    Avatar,
    StatusText,
    StatusPresence,
    ProfileContent,
    ProfileBackground,
}

#[derive(Debug, Default, Hash, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum UserHint {
    /// Could be either a user or a bot
    #[default]
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
    pub fn mention(&self) -> String {
        user_mention(&self.id)
    }
    #[inline]
    #[must_use]
    pub fn discriminator(&self) -> String {
        discriminator_display(self.discriminator)
    }
}

impl Default for User {
    /// Initializes a [`User`] with default values. Setting the following:
    /// - **[`Self::id`]** to `Id("01EZMT96C3YJ7T2NN996T8VXJE")`
    /// - **[`Self::username`]** to `foo`. 
    /// - **[`Self::discriminator`]** to `1337`.
    /// - **[`Self::display_name`]** to `foo`.
    /// - **[`Self::privileged`]** to `true`.
    /// - **[`Self::online`]** to `Some(true)`.
    fn default() -> Self {
        Self {
            id: Id::new("01EZMT96C3YJ7T2NN996T8VXJE"),
            username: "foo".to_string(),
            avatar: None,
            discriminator: 1337,
            display_name: "foo".to_string(),
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
    format!("{}#{}", name, discriminator_display(discriminator))
}

fn discriminator_display(discriminator: u16) -> String {
    format!("{discriminator:0>4?}")
}

fn user_mention(id: impl std::fmt::Display) -> String {
    format!("<@{id}>")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn discriminator_display_correctness() {
        assert_eq!("0000", discriminator_display(0));
        assert_eq!("0001", discriminator_display(1));
        assert_eq!("2484", discriminator_display(2484));
    }
}
