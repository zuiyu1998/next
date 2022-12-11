use crate::error::{Kind, ResponseResult};
use next_service::level_template::{LevelInfo, LevelTemplateQuery};
use serde::{Deserialize, Serialize};
use validator::Validate;

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
