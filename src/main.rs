mod app_settings;

use anyhow::{Context, Result};
use app_settings::Settings;
use axum::{http::StatusCode, routing::get, Router};
use sqlx::MySqlPool;
use std::net::SocketAddr;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(Debug, Clone)]
struct AppState {
    db: MySqlPool,
    config: Settings,
}

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
        .route("/health_check", get(health_check))
        .with_state(global_state);
    let addr = SocketAddr::from((config.host, config.port));

    println!("Listening on port: {}!", config.port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
