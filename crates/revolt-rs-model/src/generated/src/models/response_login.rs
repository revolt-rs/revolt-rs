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
pub struct ResponseLogin {
    #[serde(rename = "result")]
    pub result: Result,
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// Session token
    #[serde(rename = "token")]
    pub token: String,
    /// Display name
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "subscription", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Option<Box<crate::models::ResponseLoginOneOfSubscription>>>,
    #[serde(rename = "ticket")]
    pub ticket: String,
    #[serde(rename = "allowed_methods")]
    pub allowed_methods: Vec<crate::models::MfaMethod>,
}

impl ResponseLogin {
    pub fn new(result: Result, _id: String, user_id: String, token: String, name: String, ticket: String, allowed_methods: Vec<crate::models::MfaMethod>) -> ResponseLogin {
        ResponseLogin {
            result,
            _id,
            user_id,
            token,
            name,
            subscription: None,
            ticket,
            allowed_methods,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Result {
    #[serde(rename = "Disabled")]
    Disabled,
}

impl Default for Result {
    fn default() -> Result {
        Self::Disabled
    }
}

