use revolt_rs_model::{user::User, id::{UserMarker, Id}};

use crate::{client::Client, error::Errors};

type Result<T> = std::result::Result<T, Errors>;

impl Client {
    /// Get a user by their ID
    pub async fn user(&self, user: Id<UserMarker>) -> Result<User> {
        Ok(self
            .http
            .get(format!("{}{}{}", self.base_url.as_deref().unwrap().to_string(), "/users/", user.get()))
            .send() 
            .await?
            .json()
            .await?
        )
    }
}
