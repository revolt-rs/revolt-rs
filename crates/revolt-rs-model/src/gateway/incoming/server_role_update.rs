use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::server::role::{Role, RoleFields};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerRoleUpdate {
    /// Server ID
    pub id: String,
    /// Role ID
    pub role_id: String,
    /// Changed Role data
    pub data: Role,
    /// Role Fields
    pub clear: Vec<RoleFields>,
}

impl Deref for ServerRoleUpdate {
    type Target = Role;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}