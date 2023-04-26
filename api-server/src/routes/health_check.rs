use axum::extract::State;
use axum::http::StatusCode;
use mobc_redis::mobc::Pool;
use mobc_redis::redis::aio::Connection;
use mobc_redis::RedisConnectionManager;
use tracing::error;
use utils::AppState;

use crate::common::Responder;

pub async fn mysql_check(conn: &sqlx::MySqlPool) -> Result<(), anyhow::Error> {
    sqlx::query!("SELECT now() as now").fetch_one(conn).await?;
    Ok(())
}

pub async fn redis_check(pool: Pool<RedisConnectionManager>) -> Result<(), anyhow::Error> {
    let mut conn = pool.get().await?;
    _ = mobc_redis::redis::cmd("PING")
        .query_async::<_, String>(&mut conn as &mut Connection)
        .await;
    Ok(())
}

pub async fn health_check(State(app_state): State<AppState>) -> Responder<String> {
    if mysql_check(&app_state.db).await.is_err() {
        error!("Database connection failed!");
        return Responder::create_response(
            "Database Connection Failed!".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }

    if redis_check(app_state.redis_pool).await.is_err() {
        error!("Redis connection failed!");
        return Responder::create_response(
            "Redis Connection Failed!".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
            None,
        );
    }

    Responder::create_response("Ok".to_string(), StatusCode::OK, None)
}
