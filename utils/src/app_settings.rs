use crate::config::Settings;
use mobc_redis::{mobc::Pool, RedisConnectionManager};
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub config: Settings,
    pub redis_pool: Pool<RedisConnectionManager>,
}
