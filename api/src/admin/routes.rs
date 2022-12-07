use crate::middlewares::{Auth, Bearer, JwtAuth};
use next_service::Service;
use poem::{
    get, handler, post,
    web::{Data, Form, Json},
    Endpoint, EndpointExt, Route,
};
use serde_json::{json, Value};

use super::models::{LevelTemplateCreateOption, LevelTemplateQueryOption};

use next_service::users::User;

//模板列表
#[handler]
async fn find_level_templates(
    Data(service): Data<&Service>,
    Form(option): Form<LevelTemplateQueryOption>,
) -> poem::Result<Json<Value>> {
    let page_size = option.page_size;
    let mut page = option.page;
    let mut has_next = true;

    let data = super::apis::find_level_templates(service, option).await?;

    if data.len() < page_size as usize {
        has_next = false;
    }

    if has_next {
        page = page + 1;
    }

    Ok(Json(json!({
        "code": 200,
        "data": data,
        "page": page,
        "page_size": page_size,
        "has_next": has_next
    })))
}

//创建等级模板
#[handler]
async fn create_level_template(
    Data(user): Data<&User>,
    Data(service): Data<&Service>,
    Form(option): Form<LevelTemplateCreateOption>,
) -> poem::Result<Json<Value>> {
    super::apis::create_level_template(service, option).await?;
    Ok(Json(json!({
        "code": 200,
        "data": user
    })))
}

pub fn config() -> impl Endpoint {
    let auth: Auth<Bearer> = Auth::new();
    let jwt_auth = JwtAuth;

    Route::new()
        .at(
            "/create_level_template",
            post(create_level_template)
                .with(jwt_auth.clone())
                .with(auth.clone()),
        )
        .at(
            "/find_level_templates",
            post(find_level_templates)
                .with(jwt_auth.clone())
                .with(auth.clone()),
        )
}
