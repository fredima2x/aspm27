use crate::libs::models::db_objects::DirectMessage;
use sqlx::SqlitePool;

pub async fn save_message(
    owner_id: i64,
    chat_id: i64,
    content: &str,
    pool: SqlitePool,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO messages (owner_id, chat_id, content) VALUES (?, ?, ?)")
        .bind(owner_id)
        .bind(chat_id)
        .bind(content)
        .execute(&pool)
        .await?;
    Ok(result.last_insert_rowid())
}

pub async fn get_message(message_id: i64, pool: SqlitePool) -> Result<DirectMessage, sqlx::Error> {
    sqlx::query_as::<_, DirectMessage>(
        "SELECT * FROM messages WHERE id = ? AND soft_delete = FALSE",
    )
    .bind(message_id)
    .fetch_one(&pool)
    .await
}

#[allow(dead_code)]
pub async fn delete_message(message_id: i64, pool: SqlitePool) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM messages WHERE id = ? AND soft_delete = FALSE")
        .bind(message_id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

pub async fn message_soft_delete(id: i64, pool: SqlitePool) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        "UPDATE messages SET soft_delete = TRUE, deleted_at = CURRENT_TIMESTAMP WHERE id = ? AND soft_delete = FALSE",
    )
    .bind(id)
    .execute(&pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

pub async fn chat_get_messages(
    chat_id: i64,
    limit: i64,
    offset: i64,
    pool: SqlitePool,
) -> Result<Vec<DirectMessage>, sqlx::Error> {
    sqlx::query_as::<_, DirectMessage>(
        "SELECT * FROM messages WHERE chat_id = ? AND soft_delete = FALSE ORDER BY created_at ASC LIMIT ? OFFSET ?",
    )
    .bind(chat_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await
}
