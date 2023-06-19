use serde::{Deserialize, Serialize};

use crate::id::{ChannelMarker, Id, MessageMarker};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct MessageDelete {
    #[serde(rename = "_id")]
    pub id: Id<MessageMarker>,
    pub channel: Id<ChannelMarker>,
}
