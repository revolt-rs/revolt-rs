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
pub struct BuildInformation {
    /// Commit Hash
    #[serde(rename = "commit_sha")]
    pub commit_sha: String,
    /// Commit Timestamp
    #[serde(rename = "commit_timestamp")]
    pub commit_timestamp: String,
    /// Git Semver
    #[serde(rename = "semver")]
    pub semver: String,
    /// Git Origin URL
    #[serde(rename = "origin_url")]
    pub origin_url: String,
    /// Build Timestamp
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

impl BuildInformation {
    pub fn new(commit_sha: String, commit_timestamp: String, semver: String, origin_url: String, timestamp: String) -> BuildInformation {
        BuildInformation {
            commit_sha,
            commit_timestamp,
            semver,
            origin_url,
            timestamp,
        }
    }
}


