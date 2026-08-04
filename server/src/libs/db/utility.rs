use crate::libs::config::SQLITE_DB_ADDRESS;
use sqlx::SqlitePool;

pub async fn get_pool() -> SqlitePool {
    SqlitePool::connect(SQLITE_DB_ADDRESS).await.unwrap()
}
