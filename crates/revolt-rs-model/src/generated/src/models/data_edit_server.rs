/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataEditServer {
    /// Server name
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Server description
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Attachment Id for icon
    #[serde(rename = "icon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Option<String>>,
    /// Attachment Id for banner
    #[serde(rename = "banner", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub banner: Option<Option<String>>,
    /// Category structure for server
    #[serde(rename = "categories", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Option<Vec<crate::models::Category>>>,
    #[serde(rename = "system_messages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub system_messages: Option<Option<Box<crate::models::DataEditServerSystemMessages>>>,
    /// Bitfield of server flags
    #[serde(rename = "flags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Option<i32>>,
    /// Whether this server is public and should show up on [Revolt Discover](https://rvlt.gg)
    #[serde(rename = "discoverable", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub discoverable: Option<Option<bool>>,
    /// Whether analytics should be collected for this server  Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    #[serde(rename = "analytics", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub analytics: Option<Option<bool>>,
    /// Fields to remove from server object
    #[serde(rename = "remove", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remove: Option<Option<Vec<crate::models::FieldsServer>>>,
}

impl DataEditServer {
    pub fn new() -> DataEditServer {
        DataEditServer {
            name: None,
            description: None,
            icon: None,
            banner: None,
            categories: None,
            system_messages: None,
            flags: None,
            discoverable: None,
            analytics: None,
            remove: None,
        }
    }
}


