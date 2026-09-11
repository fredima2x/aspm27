use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

pub async fn get_pool(sqlite_address: String) -> SqlitePool {
    let options = SqliteConnectOptions::from_str(&sqlite_address)
        .expect("invalid database URL")
        .create_if_missing(true)
        .foreign_keys(true);

    SqlitePool::connect_with(options)
        .await
        .expect("failed to connect to database")
}
