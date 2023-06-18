use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Category {
    /// Unique ID for this category
    #[serde(rename = "id")]
    pub id: String,
    /// Title for this category
    #[serde(rename = "title")]
    pub title: String,
    /// Channels in this category
    #[serde(rename = "channels")]
    pub channels: Vec<String>,
}
