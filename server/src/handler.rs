use axum::{Json, extract::Path, http::StatusCode};

use crate::auth;
use crate::db;
use crate::models::CreateChatRequest;
use crate::models::{
    AuthenticatedUser, Chat, CreateUserResponse, LoginResponse, SendUserRequest, User,
};

// SECURITY FRAUD
// Only for debug! Must be removed in Release
pub async fn get_users() -> Json<Vec<User>> {
    let users = db::user_getall().await;
    Json(users)
}

pub async fn create_user(Json(body): Json<SendUserRequest>) -> Json<CreateUserResponse> {
    let id: i64 = db::user_create(&body.username, &body.password).await;
    Json(CreateUserResponse { id })
}

// SECURITY FRAUD
pub async fn delete_user(Path(id): Path<i64>) {
    db::user_delete(id).await;
}

pub async fn get_user(Path(id): Path<i64>) -> Json<User> {
    let user: User = db::user_get_by_id(id).await;
    Json(user)
}

pub async fn login(Json(body): Json<SendUserRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    let user: User = db::user_get_by_name(&body.username).await;
    let result: bool = auth::verify_password(&body.password, &user.password_hash);
    if result {
        Ok(Json(LoginResponse {
            token_string: auth::create_token(user.id),
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn get_chats(user: AuthenticatedUser) -> Json<Vec<Chat>> {
    Json(db::user_get_chats(user.id).await)
}

pub async fn create_chat(
    user: AuthenticatedUser,
    Json(body): Json<CreateChatRequest>,
) -> Json<Chat> {
    let chat_id: i64 = db::chat_create(&body.chat_name).await;
    db::chat_add_user(chat_id, user.id).await;
    Json(db::chat_get(chat_id).await)
}

pub async fn get_chat(Path(id): Path<i64>) -> Json<Chat> {
    Json(db::chat_get(id).await)
}

// !!!
pub async fn hi() -> &'static str {
    "Leck Eier!"
}
