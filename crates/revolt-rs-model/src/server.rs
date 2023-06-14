use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::id::{Id, UserMarker};

use super::attachment::File;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
pub struct MemberCompositeKey {
    pub server: String,
    pub user: String,
}

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Member {
    #[serde(rename = "_id")]
    pub id: MemberCompositeKey,
    /// Timestamp representing the date when the member joined.
    pub joined_at: DateTime<FixedOffset>,
    /// Nickname the user has set for the server.
    pub nickname: Option<String>,
    /// Avatar attachment of the member.
    pub avatar: Option<File>,
    /// Vector of Role ids for the given member.
    pub roles: Vec<String>,
    /// Timestamp when the user was timedout
    pub timeout: Option<DateTime<FixedOffset>>,
}

/// ServerBan : Representation of a server ban on Revolt



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerBan {
    #[serde(rename = "_id")]
    pub id: Id<UserMarker>,
    /// Reason for ban creation
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerBanner {
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Tag / bucket this file was uploaded to
    #[serde(rename = "tag")]
    pub tag: String,
    /// Original filename
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "metadata")]
    pub metadata: Box<crate::models::FileMetadata>,
    /// Raw content type of this file
    #[serde(rename = "content_type")]
    pub content_type: String,
    /// Size of this file (in bytes)
    #[serde(rename = "size")]
    pub size: i32,
    /// Whether this file was deleted
    #[serde(rename = "deleted", default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Whether this file was reported
    #[serde(rename = "reported", default, skip_serializing_if = "Option::is_none")]
    pub reported: Option<bool>,
    #[serde(rename = "message_id", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "user_id", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "server_id", default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// Id of the object this file is associated with
    #[serde(rename = "object_id", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
}
