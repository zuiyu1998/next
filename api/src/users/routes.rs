use super::models::{UserForm, UserNikeNamedUpdate, UserPasswordUpdate};
use crate::middlewares::{Auth, Bearer, JwtAuth};
use next_service::Service;
use poem::{
    get, handler, post,
    web::{Data, Form, Json},
    Endpoint, EndpointExt, Route,
};
use serde_json::{json, Value};

use next_service::users::User;

//用户修改昵称
#[handler]
async fn update_nike_name(
    Data(service): Data<&Service>,
    Data(user): Data<&User>,
    Form(user_password_update): Form<UserNikeNamedUpdate>,
) -> poem::Result<Json<Value>> {
    super::apis::update_nike_name(service, user, user_password_update).await?;

    Ok(Json(json!({
        "code": 200,
    })))
}

//用户修改密码
#[handler]
async fn update_password(
    Data(service): Data<&Service>,
    Data(user): Data<&User>,
    Form(user_password_update): Form<UserPasswordUpdate>,
) -> poem::Result<Json<Value>> {
    super::apis::update_password(service, user, user_password_update).await?;

    Ok(Json(json!({
        "code": 200,
    })))
}

//用户忘记密码
#[handler]
async fn forget_password(
    Data(service): Data<&Service>,
    Data(user): Data<&User>,
    Form(user_password_update): Form<UserPasswordUpdate>,
) -> poem::Result<Json<Value>> {
    super::apis::update_password(service, user, user_password_update).await?;

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
        "data": user_info
    })))
}

//用户登录
#[handler]
async fn login(
    Data(service): Data<&Service>,
    Form(user_form): Form<UserForm>,
) -> poem::Result<Json<Value>> {
    let user_tokon = super::apis::login(service, user_form).await?;

    Ok(Json(json!({
        "code": 200,
        "data": user_tokon
    })))
}

//创建用户
#[handler]
async fn create(
    Data(service): Data<&Service>,
    Form(user_form): Form<UserForm>,
) -> poem::Result<Json<Value>> {
    let user = super::apis::create(service, user_form).await?;

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
            "/forget_password",
            post(forget_password)
                .with(jwt_auth.clone())
                .with(auth.clone()),
        )
        .at(
            "/update_password",
            post(update_password)
                .with(jwt_auth.clone())
                .with(auth.clone()),
        )
        .at(
            "/update_nike_name",
            post(update_nike_name)
                .with(jwt_auth.clone())
                .with(auth.clone()),
        )
}
