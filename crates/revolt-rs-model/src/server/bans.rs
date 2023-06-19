use serde::{Deserialize, Serialize};

use crate::id::{Id, UserMarker};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// Representation of a server ban.
pub struct ServerBan {
    #[serde(rename = "_id")]
    pub id: Id<UserMarker>,
    /// Reason for ban creation
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BannedUser {
    /// Id of the banned user
    #[serde(rename = "_id")]
    pub id: String,
    /// Username of the banned user
    pub username: String,
    /// Avatar of the banned user
    pub avatar: Option<crate::attachment::File>,
}

#[derive(Clone, Debug, Serialize, Deserialize)] 
/// Ban list
pub struct BanList {
    /// Users objects
    pub users: Vec<BannedUser>,
    /// Ban objects
    pub bans: Vec<ServerBan>,
}