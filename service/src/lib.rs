pub use next_core as core;
pub use next_core::sea_orm;

use next_core::sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};

pub mod error;

pub mod level_template;
pub mod popularity;
pub mod users;

use error::Result;

#[derive(Debug, Clone)]
pub struct Service {
    conn: DatabaseConnection,
}

pub enum ServiceInner<'a> {
    Transaction(&'a DatabaseTransaction),
    Conn(&'a DatabaseConnection),
}

impl Service {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub fn user(&self) -> users::UserService {
        users::UserService::new_connection(&self.conn)
    }

    pub fn level_template(&self) -> level_template::LevelTemplateService {
        level_template::LevelTemplateService::new_connection(&self.conn)
    }

    pub fn popularity(&self) -> popularity::PopularityService {
        popularity::PopularityService::new_connection(&self.conn)
    }

    pub async fn begin(&self) -> Result<Transaction> {
        let begin = self.conn.begin().await?;

        Ok(Transaction(begin))
    }
}

pub struct Transaction(DatabaseTransaction);

impl Transaction {
    pub async fn commit(self) -> Result<()> {
        self.0.commit().await?;
        Ok(())
    }

    pub fn user(&self) -> users::UserService {
        users::UserService::new_transaction(&self.0)
    }

    pub fn level_template(&self) -> level_template::LevelTemplateService {
        level_template::LevelTemplateService::new_transaction(&self.0)
    }

    pub fn popularity(&self) -> popularity::PopularityService {
        popularity::PopularityService::new_transaction(&self.0)
    }
}
