use revolt_rs_model::{
    channel::message::{data::DataEditMessage, embed::SendableEmbed, reply::Reply},
    id::{ChannelMarker, Id, MessageMarker},
};

use crate::{client::Client, error::Errors};

type Result<T> = std::result::Result<T, Errors>;

#[must_use = "requests must be configured and executed"]
#[derive(Debug)]
pub struct EditMessage {
    client: Client,
    channel_id: Id<ChannelMarker>,
    message_id: Id<MessageMarker>,
    fields: DataEditMessage,
}

impl EditMessage {
    pub(crate) fn new(
        client: Client,
        channel_id: Id<ChannelMarker>,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            client,
            channel_id,
            message_id,
            fields: DataEditMessage {
                content: None,
                embeds: None,
            },
        }
    }

    pub fn content(mut self, content: &str) -> Result<Self> {
        // TODO: Check if content is valid
        self.fields.content = Some(Some(content.to_string()));
        Ok(self)
    }

    pub fn embeds(mut self, embeds: Option<Vec<SendableEmbed>>) -> Result<Self> {
        self.fields.embeds = Some(embeds);
        Ok(self)
    }

    pub async fn send(self) {
        let message = self
            .client
            .http
            .patch(format!(
                "{}/channels/{}/messages/{}",
                self.client.base_url.as_deref().unwrap().to_string(),
                self.channel_id,
                self.message_id,
            ))
            .json(&self.fields)
            .send()
            .await;
    }
}

impl Client {
    pub async fn edit_message(&self, channel_id: Id<ChannelMarker>, message_id: Id<MessageMarker>) -> EditMessage {
        EditMessage::new(self.clone(), channel_id, message_id)
    }
}
