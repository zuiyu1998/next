use crate::Result;
use figment::{providers::Env, Figment};
use serde::{Deserialize, Serialize};

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
