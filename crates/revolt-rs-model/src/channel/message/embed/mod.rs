use serde::{Deserialize, Serialize};

use self::{image::EmbedImage, media::EmbedMedia, special::EmbedSpecial, video::EmbedVideo};

pub mod image;
pub mod media;
pub mod special;
pub mod video;
pub mod provider;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmbedType {
    Website,
    Image,
    Video,
    Text,
    None
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Embed {
    #[serde(rename = "type")]
    /// Type of embed 
    pub embed_type: EmbedType,
    #[serde(rename = "url", default, skip_serializing_if = "Option::is_none")]
    /// URL for title
    pub url: Option<String>,
    #[serde(rename = "original_url", default, skip_serializing_if = "Option::is_none")]
    /// Original direct URL
    pub original_url: Option<Option<String>>,
    #[serde(rename = "special", default, skip_serializing_if = "Option::is_none")] 
    pub special: Option<Option<Box<EmbedSpecial>>>,
    #[serde(rename = "title", default, skip_serializing_if = "Option::is_none")]
    /// Title of text embed
    pub title: Option<Option<String>>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none")]
    /// Description of text embed
    pub description: Option<Option<String>>,
    #[serde(rename = "image", default, skip_serializing_if = "Option::is_none")]
    /// Image of the embed 
    pub image: Option<Option<Box<EmbedImage>>>,
    #[serde(rename = "video", default, skip_serializing_if = "Option::is_none")]
    /// Video of the embed
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
    #[serde(rename = "width", default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    /// Height of the video
    #[serde(rename = "height", default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "size", default, skip_serializing_if = "Option::is_none")]
    /// Size of the image 
    pub size: Option<crate::image::ImageSize>,
    #[serde(rename = "media", default, skip_serializing_if = "Option::is_none")]
    /// Media of the embed
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

pub struct EmbedBuilder {
    pub embed: SendableEmbed,
    pub embed_type: EmbedType,
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self {
            embed: SendableEmbed::default(),
            embed_type: EmbedType::None, 
        }
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.embed.icon_url = Some(Some(icon_url.into()));
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.embed.url = Some(Some(url.into()));
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.embed.title = Some(Some(title.into()));
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.embed.description = Some(Some(description.into()));
        self
    }

    pub fn media(mut self, media: impl Into<String>) -> Self {
        self.embed.media = Some(Some(media.into()));
        self
    }

    pub fn colour(mut self, colour: impl Into<String>) -> Self {
        self.embed.colour = Some(Some(colour.into()));
        self
    }

    pub fn build(self) -> Option<Vec<SendableEmbed>> {
        let mut embed = Vec::new();
        embed.push(self.embed);
        return Some(embed);
    }
}