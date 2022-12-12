use crate::error::{Kind, ResponseResult};
use crate::middlewares::service_auth::encode;
use next_service::popularity::PopularityCreate;
use next_service::{
    users::{User, UserFind, UserService, UserUpdate},
    Service,
};
use validator::Validate;

use super::models::{UserForm, UserInfo, UserNikeNamedUpdate, UserPasswordUpdate};

//获取用户必要信息
pub async fn info(service: &Service, user: &User) -> ResponseResult<UserInfo> {
    let begin = service.begin().await?;
    let popularity_service = begin.popularity();
    let popularity = popularity_service.find(user.id).await?;

    begin.commit().await?;

    let info = UserInfo::new(user, popularity);
    Ok(info)
}
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

    let user_create = user_form.into();

    let mut popularity_create = PopularityCreate::default();

    popularity_create.level_template_name = "user_popularity".to_owned();

    let begin = service.begin().await?;

    let user_serivice = begin.user();

    let user = user_serivice.create(user_create).await?;

    let level_template_service = begin.level_template();

    let level_controller = level_template_service
        .find(&popularity_create.level_template_name)
        .await?;

    let level = level_controller
        .current_level(0)
        .ok_or(Kind::PasswordError)?;

    popularity_create.name = level.name.to_owned();
    popularity_create.level = level.level;
    popularity_create.next_need_count = level.next_need_count;
    popularity_create.user_id = user.id;

    let popularity_service = begin.popularity();

    popularity_service.create(popularity_create).await?;

    begin.commit().await?;

    Ok(user)
}
