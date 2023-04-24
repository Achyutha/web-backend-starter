use anyhow::{Context, Result};
use app_settings::Settings;
use axum::{routing::get, Router};
use sqlx::MySqlPool;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::info;

mod app_settings;
mod commons;
mod globals;
mod services;

use crate::globals::AppState;

fn init_tracing(config: &Settings) {
    std::env::set_var("RUST_LOG", &config.log.level[..]);
    tracing_subscriber::fmt::init();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
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

    info!(
        "Connected to the database: {}",
        &config.database.database_name
    );

    let global_state = AppState {
        db: connection_pool,
        config: config.clone(),
    };

    let app = Router::new()
        .route("/health_check", get(services::health_check::health_check))
        .fallback(services::not_found::not_found)
        .with_state(global_state)
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!("{}:{}", &config.host, &config.port)[..]
        .trim()
        .parse()
        .with_context(|| {
            format!(
                "Invalid host to run the server: {}:{}",
                &config.host, &config.port
            )
        })?;

    info!("Listening on port: {}!", config.port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}
