use crate::libs::{
    auth::hash_password, models::db_objects::{BasicUser, DirectUser},
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn user_create(
    username: &str,
    password: &str,
    pool: SqlitePool,
) -> Result<Uuid, sqlx::Error> {
    let user_id: Uuid = Uuid::now_v7();
    sqlx::query("INSERT INTO users (id, username, display_name, password_hash) VALUES (?, ?, ?, ?)")
        .bind(user_id)
        .bind(username)
        .bind(username)
        .bind(hash_password(password))
        .execute(&pool)
        .await?;

    Ok(user_id)
}

pub async fn update_user(
    user: BasicUser,
    password_hash: String,
    pool: SqlitePool,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET username = ?, password_hash = ?, display_name = ? WHERE id = ?")
        .bind(&user.username)
        .bind(&password_hash)
        .bind(&user.display_name)
        .bind(&user.id)
        .execute(&pool)
        .await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn user_delete(id: Uuid, pool: SqlitePool) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

pub async fn user_soft_delete(id: Uuid, pool: SqlitePool) -> Result<(), sqlx::Error> {
    let result = sqlx::query(
        "UPDATE users SET soft_delete = TRUE, deleted_at = CURRENT_TIMESTAMP WHERE id = ?",
    )
    .bind(id)
    .execute(&pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

pub async fn user_get_by_id(id: Uuid, pool: SqlitePool) -> Result<DirectUser, sqlx::Error> {
    sqlx::query_as::<_, DirectUser>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
}

pub async fn user_get_by_name(username: &str, pool: SqlitePool) -> Result<DirectUser, sqlx::Error> {
    sqlx::query_as::<_, DirectUser>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(&pool)
        .await
}
