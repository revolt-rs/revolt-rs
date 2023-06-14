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
pub struct DataDefaultChannelPermissions {
    #[serde(rename = "permissions")]
    pub permissions: Box<crate::models::DataDefaultChannelPermissionsAnyOf1Permissions>,
}

impl DataDefaultChannelPermissions {
    pub fn new(permissions: crate::models::DataDefaultChannelPermissionsAnyOf1Permissions) -> DataDefaultChannelPermissions {
        DataDefaultChannelPermissions {
            permissions: Box::new(permissions),
        }
    }
}

