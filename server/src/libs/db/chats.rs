use sqlx::SqlitePool;

use crate::libs::models::db_objects::{BasicChat, DirectChat, DirectUser};

pub async fn user_get_chats(
    user_id: i64,
    pool: SqlitePool,
) -> Result<Vec<DirectChat>, sqlx::Error> {
    sqlx::query_as::<_, DirectChat>(
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

pub async fn chat_create(chat_name: &str, pool: SqlitePool) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO chats (chat_name) VALUES (?)")
        .bind(chat_name)
        .execute(&pool)
        .await?;
    Ok(result.last_insert_rowid())
}

///
pub async fn chat_get(chat_id: i64, pool: SqlitePool) -> Result<DirectChat, sqlx::Error> {
    sqlx::query_as::<_, DirectChat>("SELECT * FROM chats WHERE id = ? AND soft_delete = FALSE")
        .bind(chat_id)
        .fetch_one(&pool)
        .await
}

#[allow(dead_code)]
pub async fn chat_delete(chat_id: i64, pool: SqlitePool) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM chats WHERE id = ?")
        .bind(chat_id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

pub async fn chat_soft_delete(chat_id: i64, pool: SqlitePool) -> Result<(), sqlx::Error> {
    let result = sqlx::query("UPDATE chats SET soft_delete = TRUE, deleted_at = CURRENT_TIMESTAMP WHERE id = ? AND soft_delete = FALSE")
        .bind(chat_id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

#[allow(dead_code)]
pub async fn chat_add_user(
    chat_id: i64,
    user_id: i64,
    pool: SqlitePool,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO chat_members (chat_id, user_id) VALUES (?, ?)")
        .bind(chat_id)
        .bind(user_id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn chat_delete_user(
    chat_id: i64,
    user_id: i64,
    pool: SqlitePool,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM chat_members WHERE chat_id = ? AND user_id = ?")
        .bind(chat_id)
        .bind(user_id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn chat_get_members(
    chat_id: i64,
    pool: SqlitePool,
) -> Result<Vec<DirectUser>, sqlx::Error> {
    sqlx::query_as::<_, DirectUser>(
        "SELECT users.* FROM users
         INNER JOIN chat_members ON users.id = chat_members.user_id
         WHERE chat_members.chat_id = ?",
    )
    .bind(chat_id)
    .fetch_all(&pool)
    .await
}

pub async fn is_user_in_chat(
    chat_id: i64,
    user_id: i64,
    pool: SqlitePool,
) -> Result<bool, sqlx::Error> {
    let user_chats = user_get_chats(user_id, pool).await;
    match user_chats {
        Ok(msg) => Ok(msg.iter().any(|chat| chat.id == chat_id)),
        Err(e) => Err(e),
    }
}

pub async fn update_chat(chat: BasicChat, pool: SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE chats SET chat_name = ?, chat_desc = ? WHERE id = ? AND soft_delete = FALSE",
    )
    .bind(chat.chat_name)
    .bind(chat.chat_desc)
    .bind(chat.id)
    .execute(&pool)
    .await?;
    Ok(())
}
