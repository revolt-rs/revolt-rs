use serde::{Deserialize, Serialize};

use crate::id::{EmojiMarker, Id, ServerMarker, UserMarker};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parent {
    pub id: Id<ServerMarker>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmoijiCreate {
    #[serde(rename = "_id")]
    pub id: Id<EmojiMarker>,
    pub creator_id: Id<UserMarker>,
    pub animated: bool,
    pub parent: Parent,
}
