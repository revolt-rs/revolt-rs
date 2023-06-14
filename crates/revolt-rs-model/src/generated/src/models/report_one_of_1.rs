/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// ReportOneOf1 : Report was rejected



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ReportOneOf1 {
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "rejection_reason")]
    pub rejection_reason: String,
    /// ISO8601 formatted timestamp
    #[serde(rename = "closed_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<Option<String>>,
}

impl ReportOneOf1 {
    /// Report was rejected
    pub fn new(status: Status, rejection_reason: String) -> ReportOneOf1 {
        ReportOneOf1 {
            status,
            rejection_reason,
            closed_at: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Rejected")]
    Rejected,
}

impl Default for Status {
    fn default() -> Status {
        Self::Rejected
    }
}

