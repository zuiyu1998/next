use crate::error::{Kind, ResponseResult};
use crate::middlewares::service_auth::encode;
use next_service::{
    users::{User, UserCreate, UserFind, UserService},
    Service,
};
use validator::Validate;

use super::models::UserForm;

//登录
pub async fn login(service: &Service, user_form: UserForm) -> ResponseResult<String> {
    user_form.validate()?;

    let mut user_find = UserFind::default();

    user_find.email = Some(user_form.email);

    let begin = service.begin().await?;

    let user_serivice = begin.user();

    let user = user_serivice.find(user_find).await?;
    let password = UserService::spawn_password(&user_form.password);

    if password != user.password {
        return Err(Kind::PasswordError.into());
    }

    begin.commit().await?;

    let token = encode(&user.uid)?;

    Ok(token)
}

//创建
pub async fn create(service: &Service, user_form: UserForm) -> ResponseResult<User> {
    user_form.validate()?;

    let mut user_create = UserCreate::default();

    user_create.email = user_form.email;
    user_create.password = UserService::spawn_password(&user_form.password);

    let begin = service.begin().await?;

    let user_serivice = begin.user();

    let user = user_serivice.create(user_create).await?;

    begin.commit().await?;

    Ok(user)
}
