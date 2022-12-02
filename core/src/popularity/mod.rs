pub mod api;

pub use api::Api;

#[derive(Debug, Default)]
pub struct PopularityUpdate {
    pub name: Option<String>,
    pub level: Option<i32>,
    pub count: Option<i32>,
}

#[derive(Debug, Default)]
pub struct PopularityCreate {
    pub user_id: i32,
    pub level: i32,
    pub name: String,
    pub count: i32,
    pub level_template_name: String,
}

#[derive(Debug, Default)]
pub struct PopularityQuery {
    page: i32,
    page_size: i32,
}
