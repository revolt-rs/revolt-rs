use serde::{Deserialize, Serialize};

use self::{
    media::EmbedMedia,
    special::EmbedSpecial, 
    image::EmbedImage, 
    video::EmbedVideo
};

pub mod image;
pub mod media;
pub mod special;
pub mod video;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmbedType {
    #[serde(rename = "Website")]
    Website,
    #[serde(rename = "Image")]
    Image,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Text")]
    Text,
    #[serde(rename = "None")]
    None,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Embed {
    #[serde(rename = "type")]
    pub embed_type: EmbedType,
    /// URL for title
    #[serde(rename = "url", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
    /// Original direct URL
    #[serde(rename = "original_url", default, skip_serializing_if = "Option::is_none")]
    pub original_url: Option<Option<String>>,
    #[serde(rename = "special", default, skip_serializing_if = "Option::is_none")]
    pub special: Option<Option<Box<EmbedSpecial>>>,
    /// Title of text embed
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    /// Description of text embed
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<Box<EmbedImage>>>,
    #[serde(rename = "video", default, skip_serializing_if = "Option::is_none")]
    pub video: Option<Option<Box<EmbedVideo>>>,
    /// Site name
    #[serde(rename = "site_name", default, skip_serializing_if = "Option::is_none")]
    pub site_name: Option<Option<String>>,
    /// URL to icon
    #[serde(rename = "icon_url", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Option<String>>,
    /// CSS Colour
    #[serde(rename = "colour", default, skip_serializing_if = "Option::is_none")]
    pub colour: Option<Option<String>>,
    /// Width of the video
    #[serde(rename = "width")]
    pub width: i32,
    /// Height of the video
    #[serde(rename = "height")]
    pub height: i32,
    #[serde(rename = "size")]
    pub size: crate::image::ImageSize,
    #[serde(rename = "media", default, skip_serializing_if = "Option::is_none")]
    pub media: Option<Option<Box<EmbedMedia>>>,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SendableEmbed {
    #[serde(rename = "icon_url", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Option<String>>,
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "media", default, skip_serializing_if = "Option::is_none")]
    pub media: Option<Option<String>>,
    #[serde(rename = "colour", default, skip_serializing_if = "Option::is_none")]
    pub colour: Option<Option<String>>,
}
