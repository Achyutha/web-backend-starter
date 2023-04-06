use std::env;

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub hostname: [u8; 4],
    pub port: u16,
}

impl Settings {
    pub fn get_configuration() -> Result<Self, ConfigError> {
        let run_mode = env::var("MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("configs/default"))
            .add_source(File::with_name(&format!("configs/{}", run_mode)).required(false))
            .build()?;
        s.try_deserialize()
    }
}
