use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Chat {
    pub id: i64,
    pub chat_name: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Message {
    pub id: i64,
    pub owner_id: i64,
    pub chat_id: i64,
    pub content: String,
    pub created_at: String,
}
