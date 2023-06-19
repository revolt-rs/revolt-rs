use serde::Deserialize;

use crate::id::{ChannelMarker, Id, MessageMarker, UserMarker};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ChannelAck {
    #[serde(rename = "_id")]
    pub id: Id<ChannelMarker>,
    pub message_id: Id<MessageMarker>,
    pub user: Id<UserMarker>,
}
