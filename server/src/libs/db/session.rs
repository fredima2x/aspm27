use crate::libs::{db::utility::get_pool, models::db_objects::Session};

pub async fn get_session(session_id: i64) -> Result<Session, sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE id = ?")
        .bind(session_id)
        .fetch_one(&pool)
        .await
}

pub async fn get_user_sessions(user_id: i64) -> Result<Vec<Session>, sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE owner_id = ?")
        .bind(user_id)
        .fetch_all(&pool)
        .await
}

pub async fn mark_update(session_id: i64) -> Result<(), sqlx::Error> {
    let pool = get_pool().await;
    sqlx::query("UPDATE sessions SET last_update = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(session_id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn create_session(owner_id: i64) -> Result<i64, sqlx::Error> {
    let pool = get_pool().await;
    let result = sqlx::query("INSERT INTO sessions (owner_id) VALUES (?)")
        .bind(owner_id)
        .execute(&pool)
        .await?;
    Ok(result.last_insert_rowid())
}
