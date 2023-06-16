use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Reply {
    /// Message Id
    #[serde(rename = "id")]
    pub id: String,
    /// Whether this reply should mention the message's author
    #[serde(rename = "mention")]
    pub mention: bool,
}