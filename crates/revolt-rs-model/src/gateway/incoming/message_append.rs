use serde::Deserialize;

use super::message::MessageFields;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct MessageAppend {
    /// Message ID
    pub id: String,
    /// Channel ID
    pub channel: String,
    /// Appended message information
    pub append: MessageFields,
}