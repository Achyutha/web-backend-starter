use anyhow::{Context, Result};
use app_settings::Settings;
use axum::{routing::get, Router};
use sqlx::MySqlPool;
use std::net::SocketAddr;

mod app_settings;
mod globals;
mod services;

use crate::globals::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Settings::get_configuration()
        .with_context(|| format!("Unable to read the configuration file!"))?;

    let connection_pool = MySqlPool::connect(&config.database.connection_string())
        .await
        .with_context(|| {
            format!(
                "Unable to connect to the database {}",
                &config.database.database_name
            )
        })?;

    println!("Successfully Connected to the database!");

    let global_state = AppState {
        db: connection_pool,
        config: config.clone(),
    };

    let app = Router::new()
        .route("/health_check", get(services::health_check::health_check))
        .fallback(services::not_found::not_found)
        .with_state(global_state);

    let addr = SocketAddr::from((config.host, config.port));

    println!("Listening on port: {}!", config.port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
