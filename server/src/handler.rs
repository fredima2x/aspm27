pub mod users {
    use crate::auth;
    use crate::db;
    use crate::models::{
        AuthenticatedUser, CreateUserResponse, LoginResponse, SendUserRequest, User,
    };
    use axum::{Json, extract::Path, http::StatusCode};

    pub async fn get_users() -> Json<Vec<User>> {
        let users = db::user_getall().await;
        Json(users)
    }

    pub async fn create_user(
        Json(body): Json<SendUserRequest>,
    ) -> Result<Json<CreateUserResponse>, StatusCode> {
        if body.password.len() < 8 {
            return Err(StatusCode::BAD_REQUEST);
        }
        if body.password.len() > 64 {
            return Err(StatusCode::BAD_REQUEST);
        }
        if body.username.len() < 3 {
            return Err(StatusCode::BAD_REQUEST);
        }
        if body.username.len() > 24 {
            return Err(StatusCode::BAD_REQUEST);
        }

        let id = db::user_create(&body.username, &body.password).await;
        Ok(Json(CreateUserResponse { id }))
    }

    // SECURITY FRAUD
    pub async fn delete_user(
        Path(id): Path<i64>,
        user: AuthenticatedUser,
    ) -> Result<StatusCode, StatusCode> {
        if user.id == id {
            db::user_delete(id).await;
            Ok(StatusCode::OK)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }

    pub async fn get_user(Path(id): Path<i64>) -> Json<User> {
        let user: User = db::user_get_by_id(id).await;
        Json(user)
    }

    pub async fn login(
        Json(body): Json<SendUserRequest>,
    ) -> Result<Json<LoginResponse>, StatusCode> {
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
}

pub mod chats {
    use crate::db;
    use crate::models::CreateChatRequest;
    use crate::models::{AuthenticatedUser, Chat, User};
    use axum::{Json, extract::Path, http::StatusCode};

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

    pub async fn delete_chat(
        Path(id): Path<i64>,
        user: AuthenticatedUser,
    ) -> Result<StatusCode, StatusCode> {
        if db::is_user_in_chat(id, user.id).await {
            db::chat_delete(id).await;
            Ok(StatusCode::OK)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }

    pub async fn get_chat_members(
        user: AuthenticatedUser,
        Path(id): Path<i64>,
    ) -> Result<Json<Vec<User>>, StatusCode> {
        if db::is_user_in_chat(id, user.id).await {
            Ok(Json(db::chat_get_members(id).await))
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
    pub async fn add_chat_member(
        user: AuthenticatedUser,
        Path((chat_id, user_id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, StatusCode> {
        if db::is_user_in_chat(chat_id, user.id).await {
            db::chat_add_user(chat_id, user_id).await;
            Ok(StatusCode::OK)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
    pub async fn is_user_in_chat(
        user: AuthenticatedUser,
        Path((chat_id, user_id)): Path<(i64, i64)>,
    ) -> Result<Json<bool>, StatusCode> {
        if db::is_user_in_chat(chat_id, user.id).await {
            Ok(Json(db::is_user_in_chat(chat_id, user_id).await))
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
    pub async fn remove_chat_member(
        user: AuthenticatedUser,
        Path((chat_id, user_id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, StatusCode> {
        if db::is_user_in_chat(chat_id, user.id).await {
            if db::is_user_in_chat(chat_id, user_id).await {
                db::chat_delete_user(chat_id, user_id).await;
                Ok(StatusCode::OK)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

pub mod message {
    use crate::{
        db,
        models::{AuthenticatedUser, Message, SendMessageRequest, SendUserRequest},
    };
    use axum::{Json, extract::Path, http::StatusCode};

    pub async fn save_message(
        user: AuthenticatedUser,
        Path(chat_id): Path<i64>,
        Json(body): Json<SendMessageRequest>,
    ) -> Result<Json<Message>, StatusCode> {
        if db::is_user_in_chat(chat_id, user.id).await {
            Ok(Json(
                db::get_message(db::save_message(user.id, chat_id, &body.content).await).await,
            ))
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

// !!!
pub async fn hi() -> &'static str {
    "Leck Eier!"
}
