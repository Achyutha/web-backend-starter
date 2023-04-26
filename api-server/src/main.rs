use anyhow::{Context, Result};
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use utils::config::Settings;
use utils::get_app_state;

mod common;
mod routes;

fn init_tracing(config: &Settings) {
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        &config.log.directory,
        &config.log.file_prefix,
    );
    std::env::set_var("RUST_LOG", &config.log.level[..]);
    tracing_subscriber::fmt::fmt()
        .json()
        .with_writer(file_appender)
        .init();
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
    let global_state = get_app_state().await?;

    init_tracing(&global_state.config);

    let app = Router::new()
        .route("/health_check", get(routes::health_check::health_check))
        .fallback(routes::not_found::not_found)
        .with_state(global_state.clone())
        .layer(TraceLayer::new_for_http());

    let addr: SocketAddr = format!(
        "{}:{}",
        &global_state.config.host, &global_state.config.port
    )[..]
        .trim()
        .parse()
        .with_context(|| {
            format!(
                "Invalid host to run the server: {}:{}",
                &global_state.config.host, &global_state.config.port
            )
        })?;

    info!("Listening on port: {}!", global_state.config.port);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}
