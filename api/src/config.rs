use crate::{error::ResponseResult, Result};
use figment::{providers::Env, Figment};
use next_service::{level_template::LevelTemplateCreate, Service};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EnvConfig {
    pub database_url: String,
    pub host: String,
    pub port: usize,
}

pub fn config() -> Result<EnvConfig> {
    let config: EnvConfig = Figment::new().merge(Env::prefixed("")).extract()?;

    Ok(config)
}

pub async fn init(service: &Service) {
    if let Err(e) = create_popularity_level_template(service).await {
        tracing::error!("create_popularity_level_template error is: {}", e);
    }
}

//创建基础用户等级和声望
pub async fn create_popularity_level_template(service: &Service) -> ResponseResult<()> {
    let mut create = LevelTemplateCreate::default();

    create.name = "user_popularity".to_string();

    let value = json!([{
        "level": 0,
        "name":"lv0",
        "next_need_count": 10
    },
    {
        "level": 1,
        "name":"lv1",
        "next_need_count": 20
    },
    {
        "level": 2,
        "name":"lv2",
        "next_need_count": 40
    },
    {
        "level": 3,
        "name":"lv3",
        "next_need_count": 80
    },
    {
        "level": 4,
        "name":"lv4",
        "next_need_count": 160
    },
    {
        "level": 5,
        "name":"lv5",
        "next_need_count": 320
    },
    {
        "level": 6,
        "name":"lv6",
        "next_need_count": 640
    },
    {
        "level": 7,
        "name":"lv7",
        "next_need_count": 1280
    },
    {
        "level": 8,
        "name":"lv8",
        "next_need_count": 2560
    },
    {
        "level": 9,
        "name":"lv9",
        "next_need_count": 5120
    },
    {
        "level": 10,
        "name":"max",
        "next_need_count": 10240
    }
    ]);

    let content = serde_json::to_string(&value).unwrap();

    create.content = content;

    let level_template_service = service.level_template();

    level_template_service.create(create).await?;

    Ok(())
}
