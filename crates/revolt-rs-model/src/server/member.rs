use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::user::User;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct MemberCompositeKey {
    pub server: String,
    pub user: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MemberList {
    /// List of members
    pub members: Vec<Member>,
    /// List of users
    pub users: Vec<User>,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[non_exhaustive]
/// [`User`] that is in a [`Server`].
pub struct Member {
    #[serde(rename = "_id")]
    pub id: MemberCompositeKey,
    /// Timestamp representing the date when the member joined.
    pub joined_at: DateTime<FixedOffset>,
    /// Nickname the user has set for the server.
    pub nickname: Option<String>,
    /// Avatar attachment of the member.
    pub avatar: Option<crate::attachment::File>,
    /// Vector of Role ids for the given member.
    pub roles: Vec<String>,
    /// Timestamp when the user was timedout
    pub timeout: Option<DateTime<FixedOffset>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum MemberFields {
    Nickname,
    Avatar,
    Roles,
    Timeout,
}