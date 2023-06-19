use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RolePermissions {
    /// Allow bit flags
    #[serde(rename = "a")]
    pub a: i64,
    /// Disallow bit flags
    #[serde(rename = "d")]
    pub d: i64,
}

impl RolePermissions {
    /// Permissions available to this role
    pub fn new(a: i64, d: i64) -> RolePermissions {
        RolePermissions { a, d }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Role {
    /// Role name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "permissions")]
    pub permissions: Box<RolePermissions>,
    /// Colour used for this role  This can be any valid CSS colour
    #[serde(rename = "colour", default, skip_serializing_if = "Option::is_none")]
    pub colour: Option<Option<String>>,
    /// Whether this role should be shown separately on the member sidebar
    #[serde(rename = "hoist", skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    /// Ranking of this role
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RoleFields {
    Colour,
}
