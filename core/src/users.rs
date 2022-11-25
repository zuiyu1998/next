pub mod api;

pub use api::Api;

#[derive(Debug, Default)]
pub struct UserCreate {
    pub email: String,
    pub password: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct UserUpdate {
    pub nike_name: Option<String>,
    pub password: Option<Vec<u8>>,
}

#[derive(Debug, Default)]
pub struct UserFind {
    pub id: Option<i32>,
    pub uid: Option<String>,
}

#[derive(Debug, Default)]
pub struct UserQuery {
    page: i32,
    page_size: i32,
}
