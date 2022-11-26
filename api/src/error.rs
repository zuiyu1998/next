use next_service::error::Error as ServiceError;
use poem::error::ResponseError;
use thiserror::Error;
use validator::ValidationErrors;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    ServiceError(ServiceError),
    #[error("{0}")]
    ValidationErrors(ValidationErrors),
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
