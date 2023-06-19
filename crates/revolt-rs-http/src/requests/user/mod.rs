use std::ops::Deref;

use revolt_rs_model::user::User;

use crate::{client::Client, error::Errors};

type Result<T> = std::result::Result<T, Errors>;

impl Client {
    /// Fetch a user's information.
    pub async fn fetch_user(&self, id: &str) -> Result<User> {
        Ok(self
            .http
            .get(format!("{}{}{}", self.base_url.as_deref().unwrap().to_string(), "/users/", id))
            .send() 
            .await?
            .json()
            .await?
        )
    }
}
