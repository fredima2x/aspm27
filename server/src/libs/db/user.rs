use crate::libs::{auth::hash_password, db::utility::get_pool, models::db_objects::User};

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
    sqlx::query("UPDATE users SET username = ?, password_hash = ?, display_name = ? WHERE id = ?")
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&user.display_name)
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
