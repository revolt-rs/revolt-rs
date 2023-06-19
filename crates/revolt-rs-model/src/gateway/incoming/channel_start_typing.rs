use serde::{Deserialize, Serialize};

use crate::id::{Id, ChannelMarker, UserMarker};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChannelStartTyping {
    pub id: Id<ChannelMarker>,
    pub user: Id<UserMarker>,
}