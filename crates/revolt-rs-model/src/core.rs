use serde::Deserialize;

#[derive(Clone, Hash, PartialEq, Eq, Debug, Deserialize)]
pub struct QueryNodeResponse {
    /// Version of this revolt instance
    #[serde(rename = "revolt")]
    pub version: String,
    /// The features enabled on this revolt instance
    pub features: QueryNodeFeatures,
    /// The websocket server url for this instance
    pub ws: String,
    /// URL pointing to the default revite client for this instance
    pub app: String,
    /// Web Push VAPID public key
    pub vapid: String,
    /// Build information for this instance
    pub build: QueryNodeBuildInformation,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Deserialize)]
pub struct QueryNodeFeatures {
    /// If this revolt instance has email verification support
    #[serde(rename = "email")]
    pub email_verification: bool,
    /// If this revolt instance is invite-only
    pub invite_only: bool,
    /// File server service configuration
    pub autumn: QueryNodeGenericServiceConfiguration,
    /// Proxy service configuration
    pub january: QueryNodeGenericServiceConfiguration,
    /// Voice service configuration
    pub voso: QueryNodeVoiceServiceConfiguration,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Deserialize)]
pub struct QueryNodeCaptcha {
    /// If this revolt instance requires a CAPTCHA
    pub enabled: bool,
    /// Public CAPTCHA key- if [`enabled`] is false, this may be invalid
    pub key: String,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Deserialize)]
pub struct QueryNodeGenericServiceConfiguration {
    /// If this service is enabled for this instance
    pub enabled: bool,
    /// URL of this service for this instance
    pub url: String,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Deserialize)]
pub struct QueryNodeVoiceServiceConfiguration {
    /// If this service is enabled for this instance
    pub enabled: bool,
    /// URL pointing to the voice API
    pub url: String,
    /// URL pointing to the voice WebSocket server
    pub ws: String,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, Deserialize)]
pub struct QueryNodeBuildInformation {
    /// If this revolt instance has email verification support
    #[serde(rename = "email")]
    pub email_verification: bool,
    /// If this revolt instance is invite-only
    pub invite_only: bool,
}
