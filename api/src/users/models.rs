use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserForm {
    #[validate(length(min = 1, max = 100))]
    pub email: String,
    #[validate(length(min = 8, max = 16))]
    pub password: String,
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
