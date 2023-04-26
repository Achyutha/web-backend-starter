pub mod app_settings;
pub mod config;

pub use app_settings::*;
use sqlx::MySqlPool;

use crate::config::Settings;
use anyhow::{Context, Result};

pub async fn get_app_state() -> Result<AppState, anyhow::Error> {
    let config = Settings::get_configuration()
        .with_context(|| "Unable to read the configuration file!".to_string())?;

    let connection_pool = MySqlPool::connect(&config.database.connection_string())
        .await
        .with_context(|| {
            format!(
                "Unable to connect to the database {}",
                &config.database.database_name
            )
        })?;

    Ok(AppState {
        db: connection_pool,
        config: config.clone(),
    })
}
