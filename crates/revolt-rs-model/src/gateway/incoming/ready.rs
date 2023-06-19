use serde::{Deserialize, Serialize};

use crate::{user::User, server::{Server, member::Member, emoji::Emoji}, channel::channel_type::ChannelType};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ready {
    /// List of users
    pub users: Vec<User>,
    /// List of servers
    pub servers: Vec<Server>,
    /// List of channels
    pub channels: Vec<ChannelType>,
    /// List of server members
    pub members: Vec<Member>,
    /// List of emojis
    pub emojis: Option<Vec<Emoji>>,
}