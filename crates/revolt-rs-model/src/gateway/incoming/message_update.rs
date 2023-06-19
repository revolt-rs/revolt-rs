use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{id::{Id, MessageMarker, ChannelMarker}, channel::message::PartialMessage};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct MessageUpdate {
    #[serde(rename = "_id")]
    pub id: Id<MessageMarker>,
    pub channel: Id<ChannelMarker>,
    pub data: PartialMessage,
}

impl Deref for MessageUpdate {
    type Target = PartialMessage;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}