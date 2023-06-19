use serde::{Deserialize, Serialize};

use crate::id::{ChannelMarker, EmojiMarker, Id, MessageMarker, UserMarker};

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum EmojiId {
    Id {
        /// Emoji identifier.
        id: Id<EmojiMarker>,
    },
    Unicode {
        /// Unicode name identifier.
        name: String,
    },
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum ReactionType {
    React,
    Unreact,
}

#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct Reaction {
    /// Channel identifier.
    pub channel_id: Id<ChannelMarker>,
    /// Emoji identifier.
    pub emoji_id: EmojiId,
    /// Message identifier.
    pub id: Id<MessageMarker>,
    /// Reaction type.
    pub reaction: ReactionType,
    pub user_id: Id<UserMarker>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Interactions {
    /// Reactions which should always appear and be distinct
    #[serde(rename = "reactions", default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Option<Vec<String>>>,
    /// Whether reactions should be restricted to the given list  Can only be set to true if reactions list is of at least length 1
    #[serde(rename = "restrict_reactions", skip_serializing_if = "Option::is_none")]
    pub restrict_reactions: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::{Reaction, ReactionType};
    use crate::id::{EmojiMarker, Id};

    #[test]
    fn message_reaction_id() {
        let reaction = Reaction {
            channel_id: Id::<super::ChannelMarker>::new("01H388QK3BBH3B3S46X14W84VT"),
            emoji_id: super::EmojiId::Id {
                id: Id::<EmojiMarker>::new("01GZYQS64JEW1KTX7K8PPGMVA5"),
            },
            id: Id::<super::MessageMarker>::new("01H38MCBB4ETDFMZH2DZ238MG3"),
            reaction: ReactionType::React,
            user_id: Id::<super::UserMarker>::new("01H30YN87NZCKGWQXPD5XY8JS8"),
        };
    }
    fn message_reaction_unicode() {
        let reaction = Reaction {
            channel_id: Id::<super::ChannelMarker>::new("01H388QK3BBH3B3S46X14W84VT"),
            emoji_id: super::EmojiId::Unicode { 
                name: "👍".to_string(),
            },
            id: Id::<super::MessageMarker>::new("01H38MCBB4ETDFMZH2DZ238MG3"),
            reaction: ReactionType::React,
            user_id: Id::<super::UserMarker>::new("01H30YN87NZCKGWQXPD5XY8JS8"),
        };
    }
}
