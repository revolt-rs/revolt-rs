use serde::{Deserialize, Serialize};

use crate::{
    channel::channel_type::ChannelType,
    id::{ChannelMarker, Id, ServerMarker},
};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct ChannelCreate {
    #[serde(rename = "_id")]
    pub id: Id<ChannelMarker>,
    pub server: Id<ServerMarker>,
    pub name: String,
    pub channel_type: ChannelType,
}
