use crate::config::Settings;
use sqlx::MySqlPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub config: Settings,
}
