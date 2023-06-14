/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// EmbedOneOf : Website metadata



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct EmbedOneOf {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    /// Direct URL to web page
    #[serde(rename = "url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub url: Option<Option<String>>,
    /// Original direct URL
    #[serde(rename = "original_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub original_url: Option<Option<String>>,
    #[serde(rename = "special", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub special: Option<Option<Box<crate::models::EmbedOneOfSpecial>>>,
    /// Title of website
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    /// Description of website
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    #[serde(rename = "image", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub image: Option<Option<Box<crate::models::EmbedOneOfImage>>>,
    #[serde(rename = "video", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub video: Option<Option<Box<crate::models::EmbedOneOfVideo>>>,
    /// Site name
    #[serde(rename = "site_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<Option<String>>,
    /// URL to site icon
    #[serde(rename = "icon_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<Option<String>>,
    /// CSS Colour
    #[serde(rename = "colour", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub colour: Option<Option<String>>,
}

impl EmbedOneOf {
    /// Website metadata
    pub fn new(r#type: RHashType) -> EmbedOneOf {
        EmbedOneOf {
            r#type,
            url: None,
            original_url: None,
            special: None,
            title: None,
            description: None,
            image: None,
            video: None,
            site_name: None,
            icon_url: None,
            colour: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
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

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Website
    }
}
