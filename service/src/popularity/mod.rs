use crate::{Result, ServiceInner};
use next_core::sea_orm::DatabaseConnection;
use next_core::{popularity, sea_orm::DatabaseTransaction};

pub use next_core::popularity::*;
pub use next_core::prelude::Popularity;

pub struct PopularityService<'a>(ServiceInner<'a>);

impl<'a> PopularityService<'a> {
    pub fn new_transaction(conn: &'a DatabaseTransaction) -> Self {
        PopularityService(ServiceInner::Transaction(conn))
    }

    pub fn new_connection(conn: &'a DatabaseConnection) -> Self {
        PopularityService(ServiceInner::Conn(conn))
    }

    pub async fn create(&self, popularity_create: PopularityCreate) -> Result<Popularity> {
        let popularity = match self.0 {
            ServiceInner::Transaction(c) => popularity::Api::create(c, popularity_create).await?,
            ServiceInner::Conn(c) => popularity::Api::create(c, popularity_create).await?,
        };

        Ok(popularity)
    }

    pub async fn update(&self, popularity_update: PopularityUpdate) -> Result<Popularity> {
        let popularity = match self.0 {
            ServiceInner::Transaction(c) => popularity::Api::update(c, popularity_update).await?,
            ServiceInner::Conn(c) => popularity::Api::update(c, popularity_update).await?,
        };

        Ok(popularity)
    }

    pub async fn find(&self, user_id: i32) -> Result<Popularity> {
        let popularity = match self.0 {
            ServiceInner::Transaction(c) => popularity::Api::find(c, user_id).await?.unwrap(),
            ServiceInner::Conn(c) => popularity::Api::find(c, user_id).await?.unwrap(),
        };

        Ok(popularity)
    }
}
