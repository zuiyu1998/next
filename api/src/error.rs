use crate::middlewares::{auth::AuthError, service_auth::ServiceAuthError};
use next_service::error::Error as ServiceError;
use poem::error::ResponseError;
use thiserror::Error;
use validator::ValidationErrors;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Kind {
    #[error("PasswordError")]
    PasswordError,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    ServiceError(ServiceError),
    #[error("{0}")]
    ValidationErrors(ValidationErrors),
    #[error("{0}")]
    Kind(Kind),
    #[error("{0}")]
    ServiceAuthError(ServiceAuthError),
    #[error("{0}")]
    AuthError(AuthError),
}

impl From<ServiceAuthError> for Error {
    fn from(e: ServiceAuthError) -> Self {
        Error::ServiceAuthError(e)
    }
}

impl From<AuthError> for Error {
    fn from(e: AuthError) -> Self {
        Error::AuthError(e)
    }
}

impl From<Kind> for Error {
    fn from(e: Kind) -> Self {
        Error::Kind(e)
    }
}

impl From<ValidationErrors> for Error {
    fn from(e: ValidationErrors) -> Self {
        Error::ValidationErrors(e)
    }
}

impl From<ServiceError> for Error {
    fn from(e: ServiceError) -> Self {
        Error::ServiceError(e)
    }
}

pub type ResponseResult<T> = std::result::Result<T, Error>;

impl ResponseError for Error {
    fn status(&self) -> poem::http::StatusCode {
        poem::http::StatusCode::OK
    }
}
