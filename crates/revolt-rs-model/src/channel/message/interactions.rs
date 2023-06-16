use serde::{Deserialize, Serialize};

use crate::id::{EmojiMarker, Id};

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
    /// Emoji identifier.
    pub emoji: EmojiId,
    /// Reaction type.
    pub reaction: ReactionType,
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
    use crate::id::{Id, EmojiMarker};

    #[test]
    fn message_reaction_unicode() {
        let reaction = Reaction {
            emoji: super::EmojiId::Id {
                id: Id::<EmojiMarker>::new("5f6d3d9b9c5b3c0b7c7d0b3a"),
            },
            reaction: ReactionType::React,
        };
    }
}
