use crate::libs::{
    db::utility::get_pool,
    models::db_objects::{Chat, User},
};

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
