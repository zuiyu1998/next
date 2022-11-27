use crate::error::ResponseResult;
use poem::http::header::{self, HeaderName};
use poem::{async_trait, Endpoint, IntoResponse, Middleware, Request, Response, Result};
use std::marker::PhantomData;
use std::ops::Deref;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Bearer;

impl Scheme for Bearer {
    fn scheme_name() -> &'static str {
        "Bearer"
    }
}

#[derive(Debug, Clone)]
pub struct Auth<S>(PhantomData<S>);

impl<S: Scheme> Auth<S> {
    pub fn new() -> Self {
        Auth(Default::default())
    }
}

impl<E: Endpoint, S: Scheme> Middleware<E> for Auth<S> {
    type Output = AuthImpl<E, S>;
    fn transform(&self, ep: E) -> Self::Output {
        AuthImpl {
            ep,
            _marker: Default::default(),
        }
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("HeaderValueNotFound")]
    HeaderValueNotFound,
    #[error("HeaderValueInvild")]
    HeaderValueInvild,
}

#[derive(Debug, Clone)]
pub struct Token(String);

impl Deref for Token {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Scheme: Sync + Send + 'static {
    fn header_name() -> HeaderName {
        header::AUTHORIZATION
    }

    fn scheme_name() -> &'static str;

    fn parse(req: &Request) -> ResponseResult<Token> {
        let header_value = req
            .header(Self::header_name())
            .ok_or(AuthError::HeaderValueNotFound)?;

        let mut split_n = header_value.splitn(2, ' ');

        match split_n.next() {
            Some(scheme) if scheme == Self::scheme_name() => {}
            _ => {
                return Err(AuthError::HeaderValueInvild.into());
            }
        }

        match split_n.next() {
            Some(raw) => {
                return Ok(Token(raw.to_string()));
            }
            _ => {
                return Err(AuthError::HeaderValueInvild.into());
            }
        }
    }
}

pub struct AuthImpl<E, S> {
    ep: E,
    _marker: PhantomData<S>,
}

#[async_trait]
impl<E: Endpoint, S: Scheme> Endpoint for AuthImpl<E, S> {
    type Output = Response;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        let token = S::parse(&req)?;

        req.extensions_mut().insert(token);

        let res = self.ep.call(req).await?;

        Ok(res.into_response())
    }
}
