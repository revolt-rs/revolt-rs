use serde::{Deserialize, Serialize};

use crate::id::{Id, ChannelMarker};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EndTyping {
    pub channel: Id<ChannelMarker>,
}