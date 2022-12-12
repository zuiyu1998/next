use std::fs;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};

use once_cell::sync::Lazy;

use crate::Result;

use dashmap::DashMap;
use serde::{Deserialize, Serialize};

pub static APPCONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig::load());

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AppConfig(DashMap<String, String>);

impl AppConfig {
    const PATH: &'static str = "./app_config.json";

    fn load() -> Self {
        match Self::load_() {
            Err(e) => {
                tracing::error!("AppConfig load error: {}", e);
                return Default::default();
            }
            Ok(config) => {
                return config;
            }
        }
    }

    fn load_() -> Result<Self> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(Self::PATH)?;

        let mut raw = String::new();

        let n = file.read_to_string(&mut raw).unwrap();

        if n == 0 {
            return Ok(AppConfig::default());
        }

        let map = serde_json::from_str(&raw).unwrap();

        Ok(AppConfig(map))
    }

    pub fn store(&self) {
        if let Err(e) = self.store_() {
            tracing::error!("AppConfig load error: {}", e);
        }
    }

    fn store_(&self) -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(Self::PATH)?;

        let string: String = serde_json::to_string(&self.0)?;

        file.write_all(string.as_bytes())?;

        Ok(())
    }
}

impl Deref for AppConfig {
    type Target = DashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AppConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
