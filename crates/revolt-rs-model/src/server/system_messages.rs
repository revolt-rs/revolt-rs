use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ServerSystemMessages {
    /// ID of channel to send user join messages in
    #[serde(rename = "user_joined", default, skip_serializing_if = "Option::is_none")]
    pub user_joined: Option<Option<String>>,
    /// ID of channel to send user left messages in
    #[serde(rename = "user_left", default, skip_serializing_if = "Option::is_none")]
    pub user_left: Option<Option<String>>,
    /// ID of channel to send user kicked messages in
    #[serde(rename = "user_kicked", default, skip_serializing_if = "Option::is_none")]
    pub user_kicked: Option<Option<String>>,
    /// ID of channel to send user banned messages in
    #[serde(rename = "user_banned", default, skip_serializing_if = "Option::is_none")]
    pub user_banned: Option<Option<String>>,
}
