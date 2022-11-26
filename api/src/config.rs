use crate::Result;
use figment::{providers::Env, Figment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: usize,
}

pub fn config() -> Result<Config> {
    let config: Config = Figment::new().merge(Env::prefixed("")).extract()?;

    Ok(config)
}
