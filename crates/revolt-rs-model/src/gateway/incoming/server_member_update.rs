use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::server::member::{Member, MemberCompositeKey, MemberFields};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerMemberUpdate {
    /// Member ID
    pub id: MemberCompositeKey,
    /// Member changed data
    pub data: Member,
    /// List of removed optional member fields
    pub clear: Vec<MemberFields>,
}

impl Deref for ServerMemberUpdate {
    type Target = Member;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}