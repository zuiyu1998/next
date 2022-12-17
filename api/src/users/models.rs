use serde::{Deserialize, Serialize};
use validator::Validate;

use next_service::popularity::Popularity;
use next_service::sea_orm::entity::prelude::ChronoDateTime;
use next_service::users::{User, UserCreate, UserService};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserInfo {
    pub id: i32,
    pub uid: String,
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
            email: user.email.to_owned(),
            create_at: user.create_at.to_owned(),
            update_at: user.update_at.to_owned(),
            status: user.status,
            popularity,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserUpdatePasswordRequest {
    #[validate(length(min = 8, max = 16))]
    pub old_password: String,
    #[validate(length(min = 8, max = 16))]
    pub new_password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserCreateRequest {
    #[validate(length(min = 1, max = 100), email)]
    pub email: String,
    #[validate(length(min = 8, max = 16))]
    pub password: String,
}

impl From<UserCreateRequest> for UserCreate {
    fn from(req: UserCreateRequest) -> Self {
        let mut create = UserCreate::default();

        create.email = req.email;
        create.password = UserService::spawn_password(&req.password);

        create
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserLoginRequest {
    #[validate(length(min = 1, max = 100), email)]
    pub email: String,
    #[validate(length(min = 8, max = 16))]
    pub password: String,
}

impl From<UserLoginRequest> for UserCreate {
    fn from(req: UserLoginRequest) -> Self {
        let mut create = UserCreate::default();

        create.email = req.email;
        create.password = UserService::spawn_password(&req.password);

        create
    }
}
