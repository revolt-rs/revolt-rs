use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{channel::channel_type::ChannelType, server::Server};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerCreate {
    /// Server ID
    pub id: String,
    /// Server information
    pub server: Server,
    /// List of server channels
    pub channels: Vec<ChannelType>,
}

impl Deref for ServerCreate {
    type Target = Server;

    fn deref(&self) -> &Self::Target {
        &self.server
    }
}