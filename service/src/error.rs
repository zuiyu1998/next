use next_core::sea_orm::DbErr;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Kind {
    #[error("UserAlreadyExists")]
    UserAlreadyExists,
    #[error("LevelTemplateFormateError")]
    LevelTemplateFormateError,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("{0}")]
    DbErr(DbErr),
    #[error("{0}")]
    Kind(Kind),
}

impl From<&String> for Error {
    fn from(raw: &String) -> Self {
        let duplicate_entry_regex = Regex::new(r"Duplicate entry").unwrap();

        let duplicate_entry_is_ok = duplicate_entry_regex.is_match(raw);
        let values: Vec<&str> = raw.split("'").collect();

        let table_name: String = match values.get(3) {
            None => {
                return Error::DbErr(DbErr::Exec(raw.to_owned()));
            }

            Some(str) => {
                let tmp = (*str).to_string();

                let tmps: Vec<&str> = tmp.split(".").collect();
                tmps[0].to_string()
            }
        };

        if duplicate_entry_is_ok && table_name == "users" {
            return Error::Kind(Kind::UserAlreadyExists);
        }

        return Error::DbErr(DbErr::Exec(raw.to_owned()));
    }
}

pub fn from_(e: DbErr) -> Error {
    tracing::error!("DbErr: {}", e);

    match e {
        DbErr::Exec(s) => {
            return Error::from(&s);
        }
        _ => {
            return Error::DbErr(e);
        }
    }
}

impl From<DbErr> for Error {
    fn from(e: DbErr) -> Self {
        from_(e)
    }
}

impl From<Kind> for Error {
    fn from(e: Kind) -> Self {
        Error::Kind(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

mod tests {

    #[test]
    fn from_string() {
        use super::{Error, Kind};

        let raw = "error returned from database: 1062 (23000): Duplicate entry '1542844298@qq.com' for key 'users.email'".to_string();
        let error = Error::from(&raw);

        assert_eq!(error, Error::Kind(Kind::UserAlreadyExists))
    }
}
