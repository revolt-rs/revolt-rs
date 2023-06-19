use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct Authenticated {
    #[serde(rename = "type")]
    _type: String
}
