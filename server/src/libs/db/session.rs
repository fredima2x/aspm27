use crate::libs::models::db_objects::Session;
use sqlx::SqlitePool;
use uuid::Uuid;

#[allow(dead_code)]
pub async fn get_session(session_id: Uuid, pool: SqlitePool) -> Result<Session, sqlx::Error> {
    sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE id = ?")
        .bind(session_id)
        .fetch_one(&pool)
        .await
}

#[allow(dead_code)]
pub async fn get_user_sessions(
    user_id: Uuid,
    pool: SqlitePool,
) -> Result<Vec<Session>, sqlx::Error> {
    sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE owner_id = ?")
        .bind(user_id)
        .fetch_all(&pool)
        .await
}

#[allow(dead_code)]
pub async fn mark_update(session_id: Uuid, pool: SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE sessions SET last_update = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(session_id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn create_session(owner_id: Uuid, pool: SqlitePool) -> Result<Uuid, sqlx::Error> {
    let session_id = Uuid::now_v7();
    sqlx::query("INSERT INTO sessions (id, owner_id) VALUES (?, ?)")
        .bind(session_id)
        .bind(owner_id)
        .execute(&pool)
        .await?;
    Ok(session_id)
}

pub async fn does_session_exist(session_id: Uuid, pool:  SqlitePool) -> Result<bool, sqlx::Error> {
    let result = get_session(session_id, pool).await;
    match result {
        Ok(_t) => Ok(true),
        Err(sqlx::Error::RowNotFound) => Ok(false),
        Err(t) => Err(t),
    }
}

pub async fn validate_session(session_id: Uuid, pool: SqlitePool) -> Result<bool, sqlx::Error> {
    does_session_exist(session_id, pool).await
}

#[allow(dead_code)]
pub async fn delete_session(session_id: Uuid, pool:  SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE * FROM sessions WHERE id = ?")
        .bind(session_id)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn delete_all_sessions(user_id: Uuid, pool:  SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE * FROM sessions WHERE owner_id = ?")
        .bind(user_id)
        .execute(&pool)
        .await?;
    Ok(())
}