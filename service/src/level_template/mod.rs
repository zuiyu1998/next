use crate::error::Kind;
use crate::{Result, ServiceInner};
use next_core::sea_orm::DatabaseConnection;
use next_core::{level_template, sea_orm::DatabaseTransaction};
use serde::{Deserialize, Serialize};

pub use next_core::level_template::*;
pub use next_core::prelude::LevelTemplate;

pub struct LevelTemplateService<'a>(ServiceInner<'a>);

#[derive(Debug, Deserialize, Serialize)]
pub struct LevelController {
    level_infos: Vec<LevelInfo>,
}

impl LevelController {
    pub fn from_level_template(level_template: LevelTemplate) -> Result<Self> {
        let level_infos: Vec<LevelInfo> = serde_json::from_str(&level_template.content)
            .map_err(|_| Kind::LevelTemplateFormateError)?;

        Ok(LevelController { level_infos })
    }
}

impl LevelController {
    //获取当前等级信息
    pub fn current_level(&self, level: i32) -> Option<&LevelInfo> {
        self.level_infos.iter().find(|info| {
            if info.level == level {
                return true;
            }

            return false;
        })
    }
    //下一等级信息
    pub fn next_level(&self, level: i32) -> Option<&LevelInfo> {
        self.level_infos.iter().find(|info| {
            if info.level == level {
                return true;
            }

            return false;
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LevelInfo {
    pub need_count: i32,
    pub name: String,
    pub level: i32,
}

impl<'a> LevelTemplateService<'a> {
    pub fn new_transaction(conn: &'a DatabaseTransaction) -> Self {
        LevelTemplateService(ServiceInner::Transaction(conn))
    }

    pub fn new_connection(conn: &'a DatabaseConnection) -> Self {
        LevelTemplateService(ServiceInner::Conn(conn))
    }

    pub async fn create(
        &self,
        level_template_create: LevelTemplateCreate,
    ) -> Result<LevelController> {
        let level_template = match self.0 {
            ServiceInner::Transaction(c) => {
                level_template::Api::create(c, level_template_create).await?
            }
            ServiceInner::Conn(c) => level_template::Api::create(c, level_template_create).await?,
        };

        let level_controller = LevelController::from_level_template(level_template)?;

        Ok(level_controller)
    }

    pub async fn update(
        &self,
        level_template_update: LevelTemplateUpdate,
    ) -> Result<LevelController> {
        let level_template = match self.0 {
            ServiceInner::Transaction(c) => {
                level_template::Api::update(c, level_template_update).await?
            }
            ServiceInner::Conn(c) => level_template::Api::update(c, level_template_update).await?,
        };

        let level_controller = LevelController::from_level_template(level_template)?;

        Ok(level_controller)
    }

    pub async fn find(&self, name: &str) -> Result<LevelController> {
        let level_template = match self.0 {
            ServiceInner::Transaction(c) => level_template::Api::find(c, name).await?.unwrap(),
            ServiceInner::Conn(c) => level_template::Api::find(c, name).await?.unwrap(),
        };

        let level_controller = LevelController::from_level_template(level_template)?;

        Ok(level_controller)
    }

    pub async fn query(&self, user_query: LevelTemplateQuery) -> Result<Vec<LevelTemplate>> {
        let level_template = match self.0 {
            ServiceInner::Transaction(c) => level_template::Api::query(c, user_query).await?,
            ServiceInner::Conn(c) => level_template::Api::query(c, user_query).await?,
        };

        Ok(level_template)
    }
}
