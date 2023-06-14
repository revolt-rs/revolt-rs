/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InviteJoinResponseOneOf {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// Channels in the server
    #[serde(rename = "channels")]
    pub channels: Vec<crate::models::Channel>,
    #[serde(rename = "server")]
    pub server: Box<crate::models::InviteJoinResponseOneOfServer>,
}

impl InviteJoinResponseOneOf {
    pub fn new(r#type: RHashType, channels: Vec<crate::models::Channel>, server: crate::models::InviteJoinResponseOneOfServer) -> InviteJoinResponseOneOf {
        InviteJoinResponseOneOf {
            r#type,
            channels,
            server: Box::new(server),
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Server")]
    Server,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Server
    }
}
