use poem::{Endpoint, Route};

use crate::users;

pub fn create() -> impl Endpoint {
    Route::new().nest("/api/v1/users", users::config())
}
