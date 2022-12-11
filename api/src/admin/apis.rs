use std::collections::HashMap;

use crate::{data::APPCONFIG, error::ResponseResult};
use next_service::{
    level_template::{LevelTemplate, LevelTemplateCreate, LevelTemplateQuery},
    Service,
};
use validator::Validate;

use super::models::{KeyAndValueOption, LevelTemplateCreateOption, LevelTemplateQueryOption};

//获取全局字典
pub async fn set_app_config(option: KeyAndValueOption) -> ResponseResult<()> {
    option.validate()?;

    APPCONFIG.insert(option.key, option.value);
    APPCONFIG.store();

    Ok(())
}

//获取全局字典
pub async fn get_app_config() -> ResponseResult<HashMap<String, String>> {
    let mut map = HashMap::default();

    APPCONFIG.iter().for_each(|entity| {
        map.insert(entity.key().to_string(), entity.value().to_string());
    });

    Ok(map)
}

//获取等级模板列表
pub async fn find_level_templates(
    service: &Service,
    option: LevelTemplateQueryOption,
) -> ResponseResult<Vec<LevelTemplate>> {
    option.validate()?;

    let query: LevelTemplateQuery = option.into();
    let beign = service.begin().await?;

    let level_template_service = service.level_template();

    let level_templates = level_template_service.query(query).await?;

    beign.commit().await?;

    Ok(level_templates)
}

//创建等级模板
pub async fn create_level_template(
    service: &Service,
    option: LevelTemplateCreateOption,
) -> ResponseResult<()> {
    option.validate()?;
    option.validate_json()?;

    let mut level_template_create = LevelTemplateCreate::default();

    level_template_create.name = option.name;
    level_template_create.content = option.content;

    let beign = service.begin().await?;

    let level_template_service = service.level_template();

    level_template_service.create(level_template_create).await?;

    beign.commit().await?;

    Ok(())
}
