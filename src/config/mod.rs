mod app_mode;

use anyhow::{Context, Result};
use app_mode::AppMode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub app_mode: AppMode,
    pub api_host: String,
    pub api_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_mode: AppMode::default(),
            api_host: String::from("127.0.0.1"),
            api_port: 8080,
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::from_path(".env/.env_develop").ok();
        envy::from_env().context("failed load config from environment")
    }
}
