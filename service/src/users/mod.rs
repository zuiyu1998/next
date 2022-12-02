use crate::{Result, ServiceInner};
use next_core::sea_orm::DatabaseConnection;
use next_core::{sea_orm::DatabaseTransaction, users};
use sha2::{Digest, Sha256};

pub use next_core::prelude::User;
pub use next_core::users::*;

pub struct UserService<'a>(ServiceInner<'a>);

impl<'a> UserService<'a> {
    pub fn spawn_password(raw: &str) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(b"poem_up");
        hasher.update(raw.as_bytes());

        let result = hasher.finalize().to_vec();

        result
    }

    pub fn new_transaction(conn: &'a DatabaseTransaction) -> Self {
        UserService(ServiceInner::Transaction(conn))
    }

    pub fn new_connection(conn: &'a DatabaseConnection) -> Self {
        UserService(ServiceInner::Conn(conn))
    }

    pub async fn create(&self, user_create: UserCreate) -> Result<User> {
        let user = match self.0 {
            ServiceInner::Transaction(c) => users::Api::create(c, user_create).await?,
            ServiceInner::Conn(c) => users::Api::create(c, user_create).await?,
        };

        Ok(user)
    }

    pub async fn update(&self, user_update: UserUpdate) -> Result<User> {
        let user = match self.0 {
            ServiceInner::Transaction(c) => users::Api::update(c, user_update).await?,
            ServiceInner::Conn(c) => users::Api::update(c, user_update).await?,
        };
        Ok(user)
    }

    pub async fn find(&self, user_find: UserFind) -> Result<User> {
        let user = match self.0 {
            ServiceInner::Transaction(c) => users::Api::find(c, user_find).await?.unwrap(),
            ServiceInner::Conn(c) => users::Api::find(c, user_find).await?.unwrap(),
        };
        Ok(user)
    }

    pub async fn query(&self, user_query: UserQuery) -> Result<Vec<User>> {
        let users = match self.0 {
            ServiceInner::Transaction(c) => users::Api::query(c, user_query).await?,
            ServiceInner::Conn(c) => users::Api::query(c, user_query).await?,
        };

        Ok(users)
    }
}
