use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::server::{Server, ServerFields};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerUpdate {
    /// Server ID
    pub id: String,
    /// Changed Server data
    pub data: Server,
    /// Server Fields
    pub clear: Vec<ServerFields>,
}

impl Deref for ServerUpdate {
    type Target = Server;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}