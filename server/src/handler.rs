pub mod users {
    use crate::auth;
    use crate::db;
    use crate::models::{AuthenticatedUser, LoginResponse, SendUserRequest, User};
    use axum::{Json, extract::Path, http::StatusCode};

    pub async fn get_users() -> Result<Json<Vec<User>>, StatusCode> {
        let result = db::user_getall().await;
        match result {
            Ok(users) => Ok(Json(users)),
            Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn create_user(Json(body): Json<SendUserRequest>) -> Result<Json<User>, StatusCode> {
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
        let result = db::user_create(&body.username, &body.password).await;
        match result {
            Ok(id) => match db::user_get_by_id(id).await {
                Ok(user) => Ok(Json(user)),
                Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
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

    pub async fn get_user(Path(id): Path<i64>) -> Result<Json<User>, StatusCode> {
        let result = db::user_get_by_id(id).await;
        match result {
            Ok(user) => Ok(Json(user)),
            Err(e) => Err(StatusCode::NOT_FOUND),
        }
    }

    pub async fn login(
        Json(body): Json<SendUserRequest>,
    ) -> Result<Json<LoginResponse>, StatusCode> {
        let result = db::user_get_by_name(&body.username).await;
        match result {
            Ok(user) => {
                let result: bool = auth::verify_password(&body.password, &user.password_hash);
                if result {
                    Ok(Json(LoginResponse {
                        token_string: auth::create_token(user.id),
                    }))
                } else {
                    Err(StatusCode::UNAUTHORIZED)
                }
            }
            Err(e) => Err(StatusCode::NOT_FOUND),
        }
    }
}

pub mod chats {
    use crate::db;
    use crate::models::CreateChatRequest;
    use crate::models::{AuthenticatedUser, Chat, User};
    use axum::{Json, extract::Path, http::StatusCode};

    pub async fn get_chats(user: AuthenticatedUser) -> Result<Json<Vec<Chat>>, StatusCode> {
        match db::user_get_chats(user.id).await {
            Ok(chats) => Ok(Json(chats)),
            Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn create_chat(
        user: AuthenticatedUser,
        Json(body): Json<CreateChatRequest>,
    ) -> Result<Json<Chat>, StatusCode> {
        match db::chat_create(&body.chat_name).await {
            Ok(chat_id) => match db::chat_add_user(chat_id, user.id).await {
                Ok(T) => match db::chat_get(chat_id).await {
                    Ok(chat) => Ok(Json(chat)),
                    Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                },
                Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            },
            Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn get_chat(Path(id): Path<i64>) -> Result<Json<Chat>, StatusCode> {
        match db::chat_get(id).await {
            Ok(chat) => Ok(Json(chat)),
            Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    pub async fn delete_chat(
        Path(id): Path<i64>,
        user: AuthenticatedUser,
    ) -> Result<StatusCode, StatusCode> {
        match db::is_user_in_chat(id, user.id).await {
            Ok(bool) => {
                if bool {
                    db::chat_delete(id).await;
                    Ok(StatusCode::OK)
                } else {
                    Err(StatusCode::FORBIDDEN)
                }
            }
            Err(e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
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
        models::{AuthenticatedUser, GetChatMessagesRequest, Message, SendMessageRequest},
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

    pub async fn delete_message(
        user: AuthenticatedUser,
        Path(message_id): Path<i64>,
    ) -> Result<StatusCode, StatusCode> {
        if db::get_message(message_id).await.owner_id == user.id {
            db::delete_message(message_id).await;
            Ok(StatusCode::OK)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }

    pub async fn get_message(
        user: AuthenticatedUser,
        Path(message_id): Path<i64>,
    ) -> Result<Json<Message>, StatusCode> {
        let message = db::get_message(message_id).await;
        if db::is_user_in_chat(message.chat_id, user.id).await {
            Ok(Json(message))
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }

    pub async fn get_chat_messages(
        user: AuthenticatedUser,
        Path(chat_id): Path<i64>,
        Json(body): Json<GetChatMessagesRequest>,
    ) -> Result<Json<Vec<Message>>, StatusCode> {
        let messages = db::chat_get_messages(chat_id, body.limit, body.offset).await;
        if db::is_user_in_chat(chat_id, user.id).await {
            Ok(Json(messages))
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

// !!!
pub async fn hi() -> &'static str {
    "Leck Eier!"
}
