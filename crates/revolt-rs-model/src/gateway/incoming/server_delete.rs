use serde::{Deserialize, Serialize};

use crate::id::{Id, ServerMarker};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerDelete {
    /// Server ID
    pub id: Id<ServerMarker>,
}
