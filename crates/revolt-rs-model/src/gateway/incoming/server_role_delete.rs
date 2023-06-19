use serde::{Deserialize, Serialize};

use crate::id::{Id, ServerMarker};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerRoleDelete {
    /// Server ID
    pub id: Id<ServerMarker>,
    /// Role ID
    pub role_id: String,
}