use crate::error::{Kind, ResponseResult};
use crate::middlewares::service_auth::encode;
use next_service::{
    users::{User, UserCreate, UserFind, UserService, UserUpdate},
    Service,
};
use validator::Validate;

use super::models::{UserForm, UserNikeNamedUpdate, UserPasswordUpdate};

// 修改昵称
pub async fn update_nike_name(
    service: &Service,
    user: &User,
    user_nike_named_update: UserNikeNamedUpdate,
) -> ResponseResult<User> {
    user_nike_named_update.validate()?;

    if user.nike_name == user_nike_named_update.nike_name {
        return Err(Kind::SameName.into());
    }

    let mut user_update = UserUpdate::default();

    user_update.nike_name = Some(user_nike_named_update.nike_name);

    let begin = service.begin().await?;

    let user_serivice = begin.user();

    let user = user_serivice.update(user_update).await?;

    begin.commit().await?;

    Ok(user)
}

// 修改密码
pub async fn update_password(
    service: &Service,
    user: &User,
    user_password_update: UserPasswordUpdate,
) -> ResponseResult<User> {
    user_password_update.validate()?;

    let old_password = UserService::spawn_password(&user_password_update.old_password);

    if user.password != old_password {
        return Err(Kind::PasswordError.into());
    }
    let new_password = UserService::spawn_password(&user_password_update.new_password);

    let mut user_update = UserUpdate::default();

    user_update.password = Some(new_password);

    let begin = service.begin().await?;

    let user_serivice = begin.user();

    let user = user_serivice.update(user_update).await?;

    begin.commit().await?;

    Ok(user)
}

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

//创建用户
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
