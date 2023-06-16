use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Image {
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
    pub size: ImageSize,
}

/// Image positioning and size
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ImageSize {
    #[serde(rename = "Large")]
    Large,
    #[serde(rename = "Preview")]
    Preview,
}

impl ToString for ImageSize {
    fn to_string(&self) -> String {
        match self {
            Self::Large => String::from("Large"),
            Self::Preview => String::from("Preview"),
        }
    }
}

impl Default for ImageSize {
    fn default() -> ImageSize {
        Self::Large
    }
}
