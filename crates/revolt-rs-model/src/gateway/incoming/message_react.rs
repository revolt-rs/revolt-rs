use std::ops::Deref;

use serde::Deserialize;

use crate::{
    channel::message::interactions::{EmojiId, Reaction},
    id::{ChannelMarker, Id, MessageMarker, UserMarker},
};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct MessageReact {
    pub channel_id: Id<ChannelMarker>,
    pub emoji_id: EmojiId,
    pub id: Id<MessageMarker>,
    pub user_id: Id<UserMarker>,
}