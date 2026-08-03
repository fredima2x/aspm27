use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

// Data Structures
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Chat {
    pub id: i64,
    pub chat_name: String,
}

//#[derive(sqlx::FromRow)]
//pub struct ChatMembership {
//    pub chat_id: i64,
//    pub user_id: i64,
//}

#[derive(FromRow, Serialize)]
pub struct Message {
    pub id: i64,
    pub owner_id: i64,
    pub chat_id: i64,
    pub content: String,
    pub created_at: String,
}

pub struct AuthenticatedUser {
    pub id: i64,
}

// Request Structures
#[derive(Deserialize)]
pub struct SendUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

#[derive(Deserialize)]
pub struct GetChatMessagesRequest {
    pub limit: i64,
    pub offset: i64,
}

// Response Structures
#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i64,
}

#[derive(Deserialize)]
pub struct CreateChatRequest {
    pub chat_name: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token_string: String,
}
