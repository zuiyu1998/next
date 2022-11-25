pub use next_core as core;
pub use next_core::sea_orm;

use next_core::sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};

pub mod error;

pub mod users;

use error::Result;

#[derive(Debug, Clone)]
pub struct Service {
    conn: DatabaseConnection,
}

impl Service {
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
}
