pub mod api;

pub use api::Api;

#[derive(Debug, Default)]
pub struct LevelTemplateUpdate {
    pub name: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Default)]
pub struct LevelTemplateCreate {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Default)]
pub struct LevelTemplateQuery {
    page: i32,
    page_size: i32,
}
