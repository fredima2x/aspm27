use axum::{Json, extract::Path, http::StatusCode};

use crate::auth;
use crate::db;
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
pub async fn create_chat() {}
pub async fn get_chat() {}

// !!!
pub async fn hi() -> &'static str {
    "Leck Eier!"
}
