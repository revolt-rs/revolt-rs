use revolt_rs_model::errors::ErrorType;

#[derive(Debug, thiserror::Error)]
pub enum Errors {
    /// Data serialization/deserialization error
    #[error("Serde JSON serialization/deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// HTTP error
    #[error("HttpError: {0}")]
    HttpRequest(#[from] reqwest::Error),

    /// An error returned from Revolt API
    #[error("Error returned from API")]
    Api(ErrorType),
}