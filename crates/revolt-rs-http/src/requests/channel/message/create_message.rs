use revolt_rs_model::{
    channel::message::{data::DataMessageSend, embed::SendableEmbed, reply::Reply, Message},
    id::{ChannelMarker, Id, MessageMarker},
};

use crate::{client::Client, error::Errors};

type Result<T> = std::result::Result<T, Errors>;

#[must_use = "requests must be configured and executed"]
#[derive(Debug)]
pub struct CreateMessage {
    client: Client,
    channel_id: Id<ChannelMarker>,
    fields: DataMessageSend,
}

impl CreateMessage {
    pub(crate) fn new(client: Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            client,
            channel_id,
            fields: DataMessageSend {
                nonce: None,
                content: None,
                attachments: None,
                replies: None,
                embeds: None,
                masquerade: None,
                interactions: None,
            },
        }
    }

    pub fn content(mut self, content: &str) -> Result<Self> {
        // TODO: Check if content is valid
        self.fields.content = Some(Some(content.to_string()));
        Ok(self)
    }

    pub fn reply(mut self, message_id: Id<MessageMarker>, mention: bool) -> Result<Self> {
        let reply = Reply {
            id: message_id.get().to_string(),
            mention: mention,
        };

        let exists = if let Some(replies) = &self.fields.replies {
            let mut replies = replies.as_ref().unwrap().clone();
            replies.push(reply);
            replies
        } else {
            vec![reply]
        };

        self.fields.replies = Some(Some(exists)); 
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
            .post(format!(
                "{}/channels/{}/messages",
                self.client.base_url.as_deref().unwrap().to_string(),
                self.channel_id
            ))
            .json(&self.fields)
            .send()
            .await;

        println!("PAYLOAD: {:?}", &self.fields); 
    }
}

impl Client {
    pub async fn create_message(&self, channel_id: Id<ChannelMarker>) -> CreateMessage {
        CreateMessage::new(self.clone(), channel_id)
    }
}
