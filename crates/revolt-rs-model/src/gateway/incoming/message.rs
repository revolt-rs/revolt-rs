use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::{
    channel::message::embed::Embed,
    id::{ChannelMarker, Id, MessageMarker, UserMarker},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: Id<MessageMarker>,
    pub author: Id<UserMarker>,
    pub channel: Id<ChannelMarker>,
    pub content: String,
    pub nonce: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MessageFields {
    /// Additional embeds to include in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,
}

impl Deref for Message {
    type Target = Id<MessageMarker>;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl DerefMut for Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.id
    }
}
