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
pub struct DataChangePassword {
    /// New password
    #[serde(rename = "password")]
    pub password: String,
    /// Current password
    #[serde(rename = "current_password")]
    pub current_password: String,
}

impl DataChangePassword {
    pub fn new(password: String, current_password: String) -> DataChangePassword {
        DataChangePassword {
            password,
            current_password,
        }
    }
}

