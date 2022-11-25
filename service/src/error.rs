use next_core::sea_orm::DbErr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    DbErr(DbErr),
}

impl From<DbErr> for Error {
    fn from(e: DbErr) -> Self {
        Error::DbErr(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
