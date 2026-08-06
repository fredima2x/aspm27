use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
    pub soft_delete: bool,
    pub deleted_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Chat {
    pub id: i64,
    pub chat_name: String,
    pub chat_desc: String,
    pub soft_delete: bool,
    pub deleted_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Message {
    pub id: i64,
    pub owner_id: i64,
    pub chat_id: i64,
    pub content: String,
    pub soft_delete: bool,
    pub deleted_at: String,
    pub created_at: String,
    pub updated_at: String,
}
