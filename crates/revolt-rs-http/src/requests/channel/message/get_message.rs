use revolt_rs_model::{
    channel::message::Message,
    id::{ChannelMarker, Id, MessageMarker},
};

use crate::{client::Client, error::Errors};

type Result<T> = std::result::Result<T, Errors>;

impl Client {
    pub async fn get_message(
        &self,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> Result<Message> {
        Ok(self
            .http
            .get(format!(
                "{}/channels/{}/messages/{}",
                self.base_url.as_deref().unwrap().to_string(),
                channel_id.get(),
                message_id.get(),
            ))
            .send()
            .await?
            .json()
            .await?)
    }
}
