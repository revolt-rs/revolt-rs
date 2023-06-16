use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Video {
    /// URL to the original video
    #[serde(rename = "url")]
    pub url: String,
    /// Width of the video
    #[serde(rename = "width")]
    pub width: i32,
    /// Height of the video
    #[serde(rename = "height")]
    pub height: i32,
}