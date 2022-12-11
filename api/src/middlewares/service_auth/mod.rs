use next_service::users::{User, UserFind};
use next_service::Service;
use poem::{async_trait, Endpoint, IntoResponse, Middleware, Request, Response, Result};
use thiserror::Error;

pub use helper::{decode, encode};

use crate::error::ResponseResult;

use super::Token;

#[derive(Debug, Clone)]
pub struct JwtAuth;

impl<E: Endpoint> Middleware<E> for JwtAuth {
    type Output = ServiceAuthImpl<E>;
    fn transform(&self, ep: E) -> Self::Output {
        ServiceAuthImpl { ep }
    }
}

#[derive(Debug, Error)]
pub enum ServiceAuthError {
    #[error("TokenNotFound")]
    TokenNotFound,
    #[error("ServiceNotFound")]
    ServiceNotFound,
    #[error("InvalidKey")]
    InvalidKey,
    #[error("SignFailed")]
    SignFailed,
    #[error("ParseFailed")]
    ParseFailed,
    #[error("MissingSubject")]
    MissingSubject,
}

pub struct ServiceAuthImpl<E> {
    ep: E,
}

async fn call_(req: &Request) -> ResponseResult<User> {
    let token: &Token = req
        .extensions()
        .get()
        .ok_or(ServiceAuthError::TokenNotFound)?;

    let uid = decode(&token)?;

    tracing::info!("uid is: {}", uid);

    let service: &Service = req.extensions().get().unwrap();

    let mut user_find = UserFind::default();
    user_find.uid = Some(uid);
    let user_service = service.user();

    let user = user_service.find(user_find).await?;

    Ok(user)
}

#[async_trait]
impl<E: Endpoint> Endpoint for ServiceAuthImpl<E> {
    type Output = Response;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        let user = call_(&req).await?;

        req.extensions_mut().insert(user);

        let res = self.ep.call(req).await?;

        Ok(res.into_response())
    }
}

pub mod helper {
    use super::ServiceAuthError;
    use hmac::{Hmac, Mac};
    use jwt::{RegisteredClaims, SignWithKey, VerifyWithKey};
    use sha2::Sha256;
    use tracing;

    use crate::error::ResponseResult;

    pub fn encode(raw: &str) -> ResponseResult<String> {
        tracing::info!("encode raw str: {:?}", raw);

        let claims = RegisteredClaims {
            issuer: Some("up.com".into()),
            subject: Some(raw.into()),
            ..Default::default()
        };
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"poem_up_secret_key")
            .map_err(|_e| ServiceAuthError::InvalidKey)?;

        let signed_token = claims
            .sign_with_key(&key)
            .map_err(|_e| ServiceAuthError::SignFailed)?;
        Ok(signed_token)
    }

    pub fn decode(token: &str) -> ResponseResult<String> {
        tracing::info!("decode token : {:?}", token);

        let key: Hmac<Sha256> = Hmac::new_from_slice(b"poem_up_secret_key")
            .map_err(|_e| ServiceAuthError::InvalidKey)?;
        let claims: RegisteredClaims = VerifyWithKey::verify_with_key(token, &key)
            .map_err(|_e| ServiceAuthError::ParseFailed)?;

        claims
            .subject
            .ok_or(ServiceAuthError::MissingSubject.into())
    }
}
