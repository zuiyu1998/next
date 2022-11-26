use crate::error::ResponseResult;
use next_service::{
    users::{User, UserCreate, UserService},
    Service,
};
use validator::Validate;

use super::models::UserForm;

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
