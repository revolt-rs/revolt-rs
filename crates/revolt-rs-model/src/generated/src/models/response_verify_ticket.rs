/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ResponseVerifyTicket : Authorised MFA ticket, can be used to log in



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResponseVerifyTicket {
    /// Unique Id
    #[serde(rename = "_id")]
    pub _id: String,
    /// Account Id
    #[serde(rename = "account_id")]
    pub account_id: String,
    /// Unique Token
    #[serde(rename = "token")]
    pub token: String,
    /// Whether this ticket has been validated (can be used for account actions)
    #[serde(rename = "validated")]
    pub validated: bool,
    /// Whether this ticket is authorised (can be used to log a user in)
    #[serde(rename = "authorised")]
    pub authorised: bool,
    /// TOTP code at time of ticket creation
    #[serde(rename = "last_totp_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_totp_code: Option<Option<String>>,
}

impl ResponseVerifyTicket {
    /// Authorised MFA ticket, can be used to log in
    pub fn new(_id: String, account_id: String, token: String, validated: bool, authorised: bool) -> ResponseVerifyTicket {
        ResponseVerifyTicket {
            _id,
            account_id,
            token,
            validated,
            authorised,
            last_totp_code: None,
        }
    }
}


