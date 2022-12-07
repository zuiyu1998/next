use poem::{Endpoint, Route};

use crate::{admin, users};

pub fn create() -> impl Endpoint {
    Route::new()
        .nest("/api/v1/users", users::config())
        .nest("/api/v1/admin", admin::config())
}
