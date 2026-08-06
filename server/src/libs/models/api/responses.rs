use crate::libs::models::db_objects::{BasicChat, BasicUser};
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i64,
}

#[derive(Serialize)]
pub struct GetProfileResponse {
    pub user: BasicUser,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token_string: String,
}

#[derive(Serialize)]
pub struct UpdateChatResponse {
    pub new_chats: Vec<BasicChat>,
    pub deleted_chats: Vec<i64>,
}
