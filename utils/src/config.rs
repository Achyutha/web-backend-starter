use std::env;

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LogSettings {
    pub directory: String,
    pub file_prefix: String,
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisSettings {
    pub host: String,
    pub max_pool_size: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub host: String,
    pub port: u16,
    pub database: DatabaseSettings,
    pub log: LogSettings,
    pub redis: RedisSettings,
}

impl Settings {
    pub fn get_configuration() -> Result<Self, ConfigError> {
        let run_mode = env::var("MODE").unwrap_or_else(|_| "default".into());

        let s = Config::builder()
            .add_source(File::with_name("configs/default"))
            .add_source(File::with_name(&format!("configs/{}", run_mode)).required(false))
            .build()?;
        s.try_deserialize()
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

impl RedisSettings {
    pub fn connection_string(&self) -> String {
        format!("redis://{}", self.host)
    }
}
