#[derive(Debug, Default)]
pub struct UserCreate {
    pub email: String,
    pub password: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct UserUpdate {
    pub password: Option<Vec<u8>>,
    pub id: Option<i32>,
    pub uid: Option<String>,
    pub email: Option<String>,
    pub status: bool,
}

#[derive(Debug, Default)]
pub struct UserFind {
    pub id: Option<i32>,
    pub uid: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Default)]
pub struct UserQuery {
    pub page: i32,
    pub page_size: i32,
}
