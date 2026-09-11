use crate::libs::models::config::Config;
use sqlx::SqlitePool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Config,
    pub db: SqlitePool,
}
