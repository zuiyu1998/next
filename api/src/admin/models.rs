use crate::error::{Kind, ResponseResult};
use next_service::level_template::{LevelInfo, LevelTemplateQuery};
use next_service::users::{User, UserQuery};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
pub struct UserQueryRequest {
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Deserialize, Serialize, Validate, Clone, Default)]
pub struct UserQueryResponse {
    pub page: i32,
    pub page_size: i32,
    pub has_next: bool,
    pub data: Vec<User>,
}

impl From<UserQueryRequest> for UserQuery {
    fn from(req: UserQueryRequest) -> Self {
        let mut query = UserQuery::default();
        query.page = req.page;
        query.page_size = req.page_size;

        query
    }
}

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
pub struct KeyAndValueOption {
    #[validate(length(min = 1, max = 200))]
    pub key: String,
    #[validate(length(min = 1, max = 200))]
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, Clone)]
pub struct LevelTemplateQueryOption {
    pub page: i32,
    pub page_size: i32,
    #[validate(length(min = 1, max = 20))]
    pub name: Option<String>,
    pub status: Option<bool>,
}

impl From<LevelTemplateQueryOption> for LevelTemplateQuery {
    fn from(option: LevelTemplateQueryOption) -> Self {
        let mut query = LevelTemplateQuery::default();

        query.page = option.page;
        query.page_size = option.page_size;

        query.name = option.name;

        if let Some(status) = option.status {
            query.status = status;
        }

        return query;
    }
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LevelTemplateCreateOption {
    #[validate(length(min = 1, max = 20))]
    pub name: String,
    pub content: String,
}

impl LevelTemplateCreateOption {
    pub fn validate_json(&self) -> ResponseResult<()> {
        let level_infos: Vec<LevelInfo> =
            serde_json::from_str(&self.content).map_err(|_| Kind::FormatError)?;

        for level_info in level_infos.iter() {
            level_info.validate()?;
        }

        Ok(())
    }
}
