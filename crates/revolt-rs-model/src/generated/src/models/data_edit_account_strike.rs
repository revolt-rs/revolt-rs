/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.1
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// DataEditAccountStrike : New strike information



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DataEditAccountStrike {
    /// New attached reason
    #[serde(rename = "reason")]
    pub reason: String,
}

impl DataEditAccountStrike {
    /// New strike information
    pub fn new(reason: String) -> DataEditAccountStrike {
        DataEditAccountStrike {
            reason,
        }
    }
}


