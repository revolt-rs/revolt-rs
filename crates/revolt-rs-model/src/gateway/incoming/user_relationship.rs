use serde::{Deserialize, Serialize};

use crate::{
    id::{Id, UserMarker},
    user::{RelationshipStatus, User},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserRelationship {
    /// Relationship ID
    pub id: Id<UserMarker>,
    /// User with whom relationship changed
    pub user: User,
    /// New relationship status
    pub status: RelationshipStatus,
}
