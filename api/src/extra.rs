use poem::{
    async_trait,
    web::{Form, Json},
    FromRequest, Request, RequestBody, Result,
};
use serde::de::DeserializeOwned;

pub struct FormOrJson<T>(pub T);

#[async_trait]
impl<'a, T: DeserializeOwned> FromRequest<'a> for FormOrJson<T> {
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self> {
        match Form::<T>::from_request(req, body).await {
            Ok(Form(t)) => {
                return Ok(FormOrJson(t));
            }
            Err(_) => {}
        }

        match Json::<T>::from_request(req, body).await {
            Ok(Json(t)) => {
                return Ok(FormOrJson(t));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
