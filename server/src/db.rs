use sqlx::SqlitePool;

use crate::auth::hash_password;
use crate::config::SQLITE_DB_ADDRESS;
use crate::models::{Chat, Message, User};

/// = Please Add Error Handling

pub async fn setup() {
    let pool = get_pool().await;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            username      TEXT NOT NULL UNIQUE,
            display_name  TEXT NOT NULL,
            password_hash TEXT NOT NULL
        )",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS chats (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            chat_name     TEXT NOT NULL
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

///
async fn get_pool() -> SqlitePool {
    SqlitePool::connect(SQLITE_DB_ADDRESS).await.unwrap()
}

pub async fn user_getall() -> Result<Vec<User>, sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
}

pub async fn user_create(username: &str, password: &str) -> Result<i64, sqlx::Error> {
    let pool = get_pool().await;
    let result =
        sqlx::query("INSERT INTO users (username, display_name, password_hash) VALUES (?, ?, ?)")
            .bind(username)
            .bind(username)
            .bind(hash_password(password))
            .execute(&pool)
            .await?;

    Ok(result.last_insert_rowid())
}

pub async fn update_user(user: User) -> Result<(), sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query("UPDATE users SET username = ?, password_hash = ? display_name = ? WHERE id = ?")
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&user.username)
        .bind(&user.id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn user_delete(id: i64) -> Result<(), sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn user_get_by_id(id: i64) -> Result<User, sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
}

pub async fn user_get_by_name(username: &str) -> Result<User, sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(&pool)
        .await
}

///
pub async fn user_get_chats(user_id: i64) -> Result<Vec<Chat>, sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query_as::<_, Chat>(
        r#"
        SELECT c.*
        FROM chats c
        INNER JOIN chat_members cm
            ON c.id = cm.chat_id
        WHERE cm.user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
}

pub async fn chat_create(chat_name: &str) -> Result<i64, sqlx::Error> {
    let pool = get_pool().await;
    let result = sqlx::query("INSERT INTO chats (chat_name) VALUES (?)")
        .bind(chat_name)
        .execute(&pool)
        .await?;
    Ok(result.last_insert_rowid())
}

///
pub async fn chat_get(chat_id: i64) -> Result<Chat, sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query_as::<_, Chat>("SELECT * FROM chats WHERE id = ?")
        .bind(chat_id)
        .fetch_one(&pool)
        .await
}

pub async fn chat_delete(chat_id: i64) -> Result<(), sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query("DELETE FROM chats WHERE id = ?")
        .bind(chat_id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn chat_add_user(chat_id: i64, user_id: i64) -> Result<(), sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query("INSERT INTO chat_members (chat_id, user_id) VALUES (?, ?)")
        .bind(chat_id)
        .bind(user_id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn chat_delete_user(chat_id: i64, user_id: i64) -> Result<(), sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query("DELETE FROM chat_members WHERE chat_id = ? AND user_id = ?")
        .bind(chat_id)
        .bind(user_id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn chat_get_members(chat_id: i64) -> Result<Vec<User>, sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query_as::<_, User>(
        "SELECT users.* FROM users
         INNER JOIN chat_members ON users.id = chat_members.user_id
         WHERE chat_members.chat_id = ?",
    )
    .bind(chat_id)
    .fetch_all(&pool)
    .await
}

pub async fn is_user_in_chat(chat_id: i64, user_id: i64) -> Result<bool, sqlx::Error> {
    let user_chats = user_get_chats(user_id).await;
    match user_chats {
        Ok(msg) => Ok(msg.iter().any(|chat| chat.id == chat_id)),
        Err(e) => Err(e),
    }
}

pub async fn save_message(owner_id: i64, chat_id: i64, content: &str) -> Result<i64, sqlx::Error> {
    let pool = get_pool().await;
    let result = sqlx::query("INSERT INTO messages (owner_id, chat_id, content) VALUES (?, ?, ?)")
        .bind(owner_id)
        .bind(chat_id)
        .bind(content)
        .execute(&pool)
        .await?;
    Ok(result.last_insert_rowid())
}

pub async fn get_message(message_id: i64) -> Result<Message, sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE id = ?")
        .bind(message_id)
        .fetch_one(&pool)
        .await
}

#[allow(dead_code)]
pub async fn delete_message(message_id: i64) -> Result<(), sqlx::Error> {
    let pool = get_pool().await;
    let result = sqlx::query("DELETE FROM messages WHERE id = ?")
        .bind(message_id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

#[allow(dead_code)]
pub async fn chat_get_messages(
    chat_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Message>, sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query_as::<_, Message>(
        "SELECT * FROM messages WHERE chat_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
    )
    .bind(chat_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await
}
