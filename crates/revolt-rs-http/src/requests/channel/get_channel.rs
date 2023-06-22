use revolt_rs_model::{
    channel::channel_type::ChannelType,
    id::{ChannelMarker, Id},
};

use crate::{
    client::{Client, ResponseExt},
    error::Errors,
};

type Result<T> = std::result::Result<T, Errors>;

impl Client {
    pub async fn get_channel(&self, id: Id<ChannelMarker>) -> Result<ChannelType> {
        Ok(self
            .http
            .get(format!(
                "{}{}{}",
                self.base_url.as_deref().unwrap().to_string(),
                "/channels/",
                id.get()
            ))
            .send()
            .await?
            .process_error()
            .await?
            .json()
            .await?)
    }
}
