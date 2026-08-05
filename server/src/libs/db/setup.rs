use crate::libs::db::utility::get_pool;

pub async fn setup() {
    let pool = get_pool().await;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            username      VARCHAR(24) NOT NULL UNIQUE,
            display_name  VARCHAR(32) NOT NULL,
            password_hash TINYTEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS chats (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            chat_name     VARCHAR(24) NOT NULL,
            chat_desc     TINYTEXT TEXT
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS chat_members (
            chat_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,

            PRIMARY KEY (chat_id, user_id),

            FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            owner_id INTEGER NOT NULL,
            chat_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
}
