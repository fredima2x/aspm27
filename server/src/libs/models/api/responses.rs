use crate::libs::models::db_objects::{BasicChat, BasicUser};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct GetProfileResponse {
    pub user: BasicUser,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token_string: String,
}

#[allow(dead_code)]
#[derive(Serialize)]
pub struct UpdateChatResponse {
    pub new_chats: Vec<BasicChat>,
    pub deleted_chats: Vec<Uuid>,
}
