use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct Authenticate {
    pub token: String,
    pub user_id: Id<UserMarker>,
}
