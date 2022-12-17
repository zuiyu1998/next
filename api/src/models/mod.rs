use next_service::users::UserUpdate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
pub struct UserDeleteRequset {
    id: Option<i32>,
    #[validate(length(min = 1, max = 20))]
    uid: Option<String>,
    #[validate(length(min = 1, max = 100), email)]
    email: Option<String>,
}

impl UserDeleteRequset {
    pub fn is_valid(&self) -> bool {
        if self.id.is_none() && self.uid.is_none() && self.uid.is_none() {
            return false;
        } else {
            return true;
        }
    }
}

impl From<UserDeleteRequset> for UserUpdate {
    fn from(req: UserDeleteRequset) -> Self {
        let mut update = UserUpdate::default();

        update.id = req.id;
        update.uid = req.uid;
        update.email = req.email;

        update
    }
}
