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

#[derive(Debug)]
pub struct LevelTemplateQuery {
    pub page: i32,
    pub page_size: i32,
    pub name: Option<String>,
    pub status: bool,
}

impl Default for LevelTemplateQuery {
    fn default() -> Self {
        LevelTemplateQuery {
            page: Default::default(),
            page_size: Default::default(),
            name: Default::default(),
            status: true,
        }
    }
}
