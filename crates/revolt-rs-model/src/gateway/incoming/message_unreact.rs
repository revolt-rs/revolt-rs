use serde::Deserialize;

use crate::{
    channel::message::interactions::EmojiId,
    id::{ChannelMarker, Id, MessageMarker, UserMarker},
};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct MessageUnreact {
    pub channel_id: Id<ChannelMarker>,
    pub emoji_id: EmojiId,
    pub id: Id<MessageMarker>,
    pub user_id: Id<UserMarker>,
}
