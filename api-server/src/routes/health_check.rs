use axum::extract::State;
use axum::http::StatusCode;
use tracing::error;
use utils::AppState;

use crate::common::Responder;

pub async fn mysql_check(conn: &sqlx::MySqlPool) -> Result<(), anyhow::Error> {
    sqlx::query!("SELECT now() as now").fetch_one(conn).await?;
    Ok(())
}

pub async fn health_check(State(app_state): State<AppState>) -> Responder<()> {
    if mysql_check(&app_state.db).await.is_err() {
        error!("Database connection failed!");
        return Responder::create_response((), StatusCode::INTERNAL_SERVER_ERROR, None);
    }

    Responder::create_response((), StatusCode::OK, None)
}
