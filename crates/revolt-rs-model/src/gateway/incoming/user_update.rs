use serde::{Deserialize, Serialize};

use crate::{user::{User, UserFields}, id::{Id, UserMarker}};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserUpdate {
    /// User ID
    pub id: Id<UserMarker>,
    /// Changed user data
    pub data: User,
    /// Removed user fields
    pub clear: Vec<UserFields>,
}