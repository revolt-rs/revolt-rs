use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EmbedImage{
    /// URL to the original image
    #[serde(rename = "url")]
    pub url: String,
    /// Width of the image
    #[serde(rename = "width")]
    pub width: i32,
    /// Height of the image
    #[serde(rename = "height")]
    pub height: i32,
    #[serde(rename = "size")]
    pub size: crate::image::ImageSize,
}
