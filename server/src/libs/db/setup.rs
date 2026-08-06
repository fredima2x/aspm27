use crate::libs::db::utility::get_pool;

pub async fn setup() {
    let pool = get_pool().await;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sessions (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            owner_id      INTEGER NOT NULL,
            last_update   TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            username      VARCHAR(24) NOT NULL UNIQUE,
            display_name  VARCHAR(32) NOT NULL,
            password_hash TINYTEXT NOT NULL,

            soft_delete   BOOLEAN NOT NULL DEFAULT FALSE,
            deleted_at    TIMESTAMP NULL DEFAULT NULL,
            created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS chats (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            chat_name     VARCHAR(24) NOT NULL,
            chat_desc     TINYTEXT TEXT,

            soft_delete   BOOLEAN NOT NULL DEFAULT FALSE,
            deleted_at    TIMESTAMP NULL DEFAULT NULL,
            created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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

            FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (chat_id) REFERENCES chats(id) ON DELETE CASCADE,

            soft_delete   BOOLEAN NOT NULL DEFAULT FALSE,
            deleted_at    TIMESTAMP NULL DEFAULT NULL,
            created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "CREATE TRIGGER IF NOT EXISTS trg_users_updated_at
         AFTER UPDATE ON users
         BEGIN
             UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
         END",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "CREATE TRIGGER IF NOT EXISTS trg_chats_updated_at
         AFTER UPDATE ON chats
         BEGIN
             UPDATE chats SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
         END",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "CREATE TRIGGER IF NOT EXISTS trg_messages_updated_at
         AFTER UPDATE ON messages
         BEGIN
             UPDATE messages SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
         END",
    )
    .execute(&pool)
    .await
    .unwrap();
}
