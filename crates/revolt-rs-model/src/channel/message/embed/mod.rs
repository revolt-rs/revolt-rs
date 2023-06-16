use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Embed {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// URL for title
    #[serde(rename = "url", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
    /// Original direct URL
    #[serde(rename = "original_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub original_url: Option<Option<String>>,
    #[serde(rename = "special", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub special: Option<Option<Box<crate::models::EmbedOneOfSpecial>>>,
    /// Title of text embed
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    /// Description of text embed
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "image", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<Box<crate::models::EmbedOneOfImage>>>,
    #[serde(rename = "video", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub video: Option<Option<Box<crate::models::EmbedOneOfVideo>>>,
    /// Site name
    #[serde(rename = "site_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<Option<String>>,
    /// URL to icon
    #[serde(rename = "icon_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Option<String>>,
    /// CSS Colour
    #[serde(rename = "colour", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub colour: Option<Option<String>>,
    /// Width of the video
    #[serde(rename = "width")]
    pub width: i32,
    /// Height of the video
    #[serde(rename = "height")]
    pub height: i32,
    #[serde(rename = "size")]
    pub size: crate::models::ImageSize,
    #[serde(rename = "media", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub media: Option<Option<Box<crate::models::EmbedOneOf3Media>>>,
}