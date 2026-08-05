use crate::libs::config::SQLITE_DB_ADDRESS;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

pub async fn get_pool() -> SqlitePool {
    let options = SqliteConnectOptions::from_str(SQLITE_DB_ADDRESS)
        .expect("invalid database URL")
        .create_if_missing(true)
        .foreign_keys(true);

    SqlitePool::connect_with(options)
        .await
        .expect("failed to connect to database")
}
