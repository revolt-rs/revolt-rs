use serde::{Deserialize, Serialize};

use crate::id::{Id, ChannelMarker};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BeginTyping {
    pub channel: Id<ChannelMarker>,
}