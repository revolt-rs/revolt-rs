use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmbedSpecial {
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "timestamp", default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Option<String>>,
    #[serde(rename = "content_type")]
    pub content_type: BandcampType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "Streamable")]
    Streamable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BandcampType {
    #[serde(rename = "Album")]
    Album,
    #[serde(rename = "Track")]
    Track,

}

impl ToString for BandcampType {
    fn to_string(&self) -> String {
        match self {
            Self::Album => String::from("Album"),
            Self::Track => String::from("Track"),
        }
    }
}

impl Default for BandcampType {
    fn default() -> BandcampType {
        Self::Album
    }
}
