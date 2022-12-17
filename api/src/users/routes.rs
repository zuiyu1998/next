use super::models::{UserCreateRequest, UserLoginRequest, UserUpdatePasswordRequest};
use crate::extra::FormOrJson;
use crate::middlewares::{Auth, Bearer, JwtAuth};
use next_service::Service;
use poem::{
    get, handler, post,
    web::{Data, Form, Json},
    Endpoint, EndpointExt, Route,
};
use serde_json::{json, Value};

use next_service::users::User;

//用户修改密码
#[handler]
async fn update_password(
    Data(service): Data<&Service>,
    Data(user): Data<&User>,
    Form(user_update_password_req): Form<UserUpdatePasswordRequest>,
) -> poem::Result<Json<Value>> {
    super::apis::update_password(service, user, user_update_password_req).await?;

    Ok(Json(json!({
        "code": 200,
    })))
}

//用户信息
#[handler]
async fn info(Data(service): Data<&Service>, Data(user): Data<&User>) -> poem::Result<Json<Value>> {
    let user_info = super::apis::info(service, user).await?;

    Ok(Json(json!({
        "code": 200,
        "result": user_info
    })))
}

//用户登录
#[handler]
async fn login(
    Data(service): Data<&Service>,
    FormOrJson(user_login_req): FormOrJson<UserLoginRequest>,
) -> poem::Result<Json<Value>> {
    let user_tokon = super::apis::login(service, user_login_req).await?;

    Ok(Json(json!({
        "code": 200,
        "result": user_tokon
    })))
}

//创建用户
#[handler]
async fn create(
    Data(service): Data<&Service>,
    Form(user_create_req): Form<UserCreateRequest>,
) -> poem::Result<Json<Value>> {
    let user = super::apis::create(service, user_create_req).await?;

    Ok(Json(json!({
        "code": 200,
        "data": user
    })))
}

pub fn config() -> impl Endpoint {
    let auth: Auth<Bearer> = Auth::new();
    let jwt_auth = JwtAuth;

    Route::new()
        .at("/create", post(create))
        .at("/login", post(login))
        .at("/info", get(info).with(jwt_auth.clone()).with(auth.clone()))
        .at(
            "/update_password",
            post(update_password)
                .with(jwt_auth.clone())
                .with(auth.clone()),
        )
}
