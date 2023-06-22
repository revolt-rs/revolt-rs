use revolt_rs_model::user::User;

use crate::{client::Client, error::Errors};

type Result<T> = std::result::Result<T, Errors>;

impl Client {
    /// Get the current user
    pub async fn current_user(&self) -> Result<User> {
        Ok(self
            .http
            .get(format!("{}{}", self.base_url.as_deref().unwrap().to_string(), "/users/@me"))
            .send() 
            .await?
            .json()
            .await?
        )
    }
}
