use anyhow::{Context, Result};
use app_settings::Settings;
use axum::{routing::get, Router};
use sqlx::MySqlPool;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::info;

mod app_settings;
mod globals;
mod services;

use crate::globals::AppState;

fn init_tracing(config: &Settings) {
    std::env::set_var("RUST_LOG", &config.log.level[..]);
    tracing_subscriber::fmt::init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Settings::get_configuration()
        .with_context(|| format!("Unable to read the configuration file!"))?;
    
    init_tracing(&config);

    let connection_pool = MySqlPool::connect(&config.database.connection_string())
        .await
        .with_context(|| {
            format!(
                "Unable to connect to the database {}",
                &config.database.database_name
            )
        })?;

    info!("Connected to the database: {}", &config.database.database_name);

    let global_state = AppState {
        db: connection_pool,
        config: config.clone(),
    };

    let app = Router::new()
        .route("/health_check", get(services::health_check::health_check))
        .fallback(services::not_found::not_found)
        .with_state(global_state)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from((config.host, config.port));

    info!("Listening on port: {}!", config.port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
