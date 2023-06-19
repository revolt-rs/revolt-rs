use serde::{Deserialize, Serialize};

use crate::id::{Id, ServerMarker, UserMarker};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerMemberLeave {
    pub server: Id<ServerMarker>,
    pub user: Id<UserMarker>,
}
