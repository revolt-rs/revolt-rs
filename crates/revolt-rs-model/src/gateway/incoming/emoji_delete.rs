use serde::{Deserialize, Serialize};

use crate::id::{EmojiMarker, Id};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmojiDelete {
    #[serde(rename = "_id")]
    pub id: Id<EmojiMarker>,
}
