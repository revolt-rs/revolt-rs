/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// Role : Representation of a server role



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Role {
    /// Role name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "permissions")]
    pub permissions: Box<crate::models::RolePermissions>,
    /// Colour used for this role  This can be any valid CSS colour
    #[serde(rename = "colour", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub colour: Option<Option<String>>,
    /// Whether this role should be shown separately on the member sidebar
    #[serde(rename = "hoist", skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,
    /// Ranking of this role
    #[serde(rename = "rank", skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

impl Role {
    /// Representation of a server role
    pub fn new(name: String, permissions: crate::models::RolePermissions) -> Role {
        Role {
            name,
            permissions: Box::new(permissions),
            colour: None,
            hoist: None,
            rank: None,
        }
    }
}


