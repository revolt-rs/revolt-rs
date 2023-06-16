use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Masquerade {
    /// Replace the display name shown on this message
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Replace the avatar shown on this message (URL to image file)
    #[serde(rename = "avatar", default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<Option<String>>,
    /// Replace the display role colour shown on this message  
    /// 
    /// Must have `ManageRole` permission to use
    #[serde(rename = "colour", default, skip_serializing_if = "Option::is_none")]
    pub colour: Option<Option<String>>,
}