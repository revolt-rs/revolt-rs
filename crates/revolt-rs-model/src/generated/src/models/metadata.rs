/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Metadata : Metadata associated with file



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "width")]
    pub width: i32,
    #[serde(rename = "height")]
    pub height: i32,
}

impl Metadata {
    /// Metadata associated with file
    pub fn new(r#type: RHashType, width: i32, height: i32) -> Metadata {
        Metadata {
            r#type,
            width,
            height,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "File")]
    File,
    #[serde(rename = "Text")]
    Text,
    #[serde(rename = "Image")]
    Image,
    #[serde(rename = "Video")]
    Video,
    #[serde(rename = "Audio")]
    Audio,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::File
    }
}

