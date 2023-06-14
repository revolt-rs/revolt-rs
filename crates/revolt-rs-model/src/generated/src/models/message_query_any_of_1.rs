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
pub struct MessageQueryAnyOf1 {
    /// Message id before which messages should be fetched
    #[serde(rename = "before", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub before: Option<Option<String>>,
    /// Message id after which messages should be fetched
    #[serde(rename = "after", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub after: Option<Option<String>>,
    #[serde(rename = "sort", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort: Option<Option<crate::models::MessageSort>>,
}

impl MessageQueryAnyOf1 {
    pub fn new() -> MessageQueryAnyOf1 {
        MessageQueryAnyOf1 {
            before: None,
            after: None,
            sort: None,
        }
    }
}

