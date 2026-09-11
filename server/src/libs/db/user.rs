use crate::libs::{
    auth::hash_password,
    models::db_objects::{BasicUser, DirectUser},
};
use sqlx::SqlitePool;

pub async fn user_getall(pool: SqlitePool) -> Result<Vec<DirectUser>, sqlx::Error> {
    sqlx::query_as::<_, DirectUser>("SELECT * FROM users")
        .fetch_all(&pool)
        .await
}

pub async fn user_create(
    username: &str,
    password: &str,
    pool: SqlitePool,
) -> Result<i64, sqlx::Error> {
    let result =
        sqlx::query("INSERT INTO users (username, display_name, password_hash) VALUES (?, ?, ?)")
            .bind(username)
            .bind(username)
            .bind(hash_password(password))
            .execute(&pool)
            .await?;

    Ok(result.last_insert_rowid())
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
pub async fn user_delete(id: i64, pool: SqlitePool) -> Result<(), sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }
    Ok(())
}

pub async fn user_soft_delete(id: i64, pool: SqlitePool) -> Result<(), sqlx::Error> {
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

pub async fn user_get_by_id(id: i64, pool: SqlitePool) -> Result<DirectUser, sqlx::Error> {
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
