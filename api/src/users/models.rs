use serde::{Deserialize, Serialize};
use validator::Validate;

use next_service::popularity::Popularity;
use next_service::sea_orm::entity::prelude::ChronoDateTime;
use next_service::users::{User, UserCreate, UserService};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserInfo {
    pub id: i32,
    pub uid: String,
    pub nike_name: String,
    pub email: String,
    pub create_at: ChronoDateTime,
    pub update_at: ChronoDateTime,
    pub status: bool,
    pub popularity: Popularity,
}

impl UserInfo {
    pub fn new(user: &User, popularity: Popularity) -> Self {
        UserInfo {
            id: user.id,
            uid: user.uid.to_owned(),
            nike_name: user.nike_name.to_owned(),
            email: user.email.to_owned(),
            create_at: user.create_at,
            update_at: user.update_at,
            status: user.status,
            popularity,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserForm {
    #[validate(length(min = 1, max = 100))]
    pub email: String,
    #[validate(length(min = 8, max = 16))]
    pub password: String,
}

impl From<UserForm> for UserCreate {
    fn from(form: UserForm) -> Self {
        let mut user_create = UserCreate::default();

        user_create.email = form.email;
        user_create.password = UserService::spawn_password(&form.password);

        user_create
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserPasswordUpdate {
    #[validate(length(min = 8, max = 16))]
    pub new_password: String,
    #[validate(length(min = 8, max = 16))]
    pub old_password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserNikeNamedUpdate {
    #[validate(length(min = 1, max = 30))]
    pub nike_name: String,
}
