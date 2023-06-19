use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::id::{Id, ServerMarker, UserMarker};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerMemberJoin {
    pub server: Id<ServerMarker>,
    pub user: Id<UserMarker>,
}

impl Deref for ServerMemberJoin {
    type Target = Id<UserMarker>;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

#[cfg(test)]
mod tests {
    use super::ServerMemberJoin;

    use crate::id::Id;

    #[test]
    fn server_member_join() {
        let mem = ServerMemberJoin {
            server: Id::new("server"),
            user: Id::new("user"),
        };
    }
}
