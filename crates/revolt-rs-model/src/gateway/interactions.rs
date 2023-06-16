use serde::{Deserialize, Serialize};

use crate::channel::message::interactions::{EmojiId, ReactionType};
use crate::id::{ChannelMarker, Id, UserMarker};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum GatewayReactionType {
    #[serde(rename = "MessageReact")]
    MessageReact,
    #[serde(rename = "MessageUnreact")]
    MessageUnreact,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GatewayReaction {
    pub channel_id: Id<ChannelMarker>,
    pub emoji_id: ReactionType,
    pub id: EmojiId,
    #[serde(rename = "type")]
    pub reaction_type: GatewayReactionType,
    pub user_id: Id<UserMarker>,
}
