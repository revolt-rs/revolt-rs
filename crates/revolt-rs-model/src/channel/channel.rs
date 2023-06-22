use super::Channel;

// impl Channel {
//     /// Deletes this channel, returning the channel on a successful deletion.
//     ///
//     /// Returns [`HttpError`] if the user is lacking permissions.
//     #[inline]
//     pub async fn delete(self, http: Client) -> Result<Channel> {
//         http.as_ref().delete_channel(self.0).await
//     }

//     #[inline]
//     pub async fn get_channel(self, http: Client) -> Result<Channel> {
//         http.as_ref().get_channel(self.0).await
//     }

//     /// Deletes a message when an ID is provided.
//     ///
//     /// Returns [`HttpError`] if the user is lacking permissions.
//     #[inline]
//     pub async fn delete_message(self, http: Client, message_id: impl Into<String>) -> Result<()> {
//         http.as_ref().delete_message(self.0, message_id.into()).await
//     }
// }