use serde::{Deserialize, Serialize};

use crate::id::{ChannelMarker, Id};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ChannelDelete {
    #[serde(rename = "_id")]
    pub id: Id<ChannelMarker>,
}
