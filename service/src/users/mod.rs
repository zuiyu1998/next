use crate::Result;
use next_core::{sea_orm::DatabaseTransaction, users};
use sha2::{Digest, Sha256};

pub use next_core::prelude::User;
pub use next_core::users::*;

pub struct UserService<'a>(&'a DatabaseTransaction);

impl<'a> UserService<'a> {
    pub fn spawn_password(raw: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(b"poem_up");
        hasher.update(raw.as_bytes());

        let result = hasher.finalize().to_vec();

        result
    }

    pub fn new(conn: &'a DatabaseTransaction) -> Self {
        UserService(conn)
    }

    pub async fn create(&self, user_create: UserCreate) -> Result<User> {
        let user = users::Api::create(self.0, user_create).await?;
        Ok(user)
    }

    pub async fn update(&self, user_update: UserUpdate) -> Result<User> {
        let user = users::Api::update(self.0, user_update).await?;
        Ok(user)
    }

    pub async fn find(&self, user_find: UserFind) -> Result<User> {
        let user = users::Api::find(self.0, user_find).await?.unwrap();
        Ok(user)
    }

    pub async fn query(&self, user_query: UserQuery) -> Result<Vec<User>> {
        let user = users::Api::query(self.0, user_query).await?;
        Ok(user)
    }
}
