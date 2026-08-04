pub mod users {
    use crate::auth;
    use crate::auth::hash_password;
    use crate::db;
    use crate::error;
    use crate::models::GetProfileResponse;
    use crate::models::SimpleSendUserRequest;
    use crate::models::UpdateProfileRequest;
    use crate::models::{AuthenticatedUser, CreateUserResponse, LoginResponse, User};
    use axum::{Json, extract::Path, http::StatusCode};

    pub async fn get_users() -> Result<Json<Vec<User>>, StatusCode> {
        let users = db::user_getall().await.map_err(error::db_err)?;
        Ok(Json(users))
    }

    pub async fn create_user(
        Json(body): Json<SimpleSendUserRequest>,
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

        let id = db::user_create(&body.username, &body.password)
            .await
            .map_err(error::db_err)?;
        Ok(Json(CreateUserResponse { id }))
    }

    pub async fn delete_user(user: AuthenticatedUser) -> Result<StatusCode, StatusCode> {
        db::user_delete(user.id).await.map_err(error::db_err)?;
        Ok(StatusCode::OK)
    }

    pub async fn get_user(Path(id): Path<i64>) -> Result<Json<User>, StatusCode> {
        let user: User = db::user_get_by_id(id).await.map_err(error::db_err)?;
        Ok(Json(user))
    }

    pub async fn login(
        Json(body): Json<SimpleSendUserRequest>,
    ) -> Result<Json<LoginResponse>, StatusCode> {
        let user: User = db::user_get_by_name(&body.username)
            .await
            .map_err(error::db_err)?;
        let result: bool = auth::verify_password(&body.password, &user.password_hash);
        if result {
            Ok(Json(LoginResponse {
                token_string: auth::create_token(user.id),
            }))
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }

    pub async fn update_profile(
        user: AuthenticatedUser,
        Json(body): Json<UpdateProfileRequest>,
    ) -> Result<StatusCode, StatusCode> {
        db::update_user(User {
            id: user.id,
            username: body.user.username,
            display_name: body.user.display_name,
            password_hash: hash_password(&body.user.password),
        })
        .await
        .map_err(error::db_err)?;
        Ok(StatusCode::OK)
    }

    pub async fn get_profile(
        user: AuthenticatedUser,
    ) -> Result<Json<GetProfileResponse>, StatusCode> {
        Ok(Json(GetProfileResponse {
            user: db::user_get_by_id(user.id).await.map_err(error::db_err)?,
        }))
    }
}

pub mod chats {
    use crate::db;
    use crate::error;
    use crate::models::CreateChatRequest;
    use crate::models::{AuthenticatedUser, Chat, User};
    use axum::{Json, extract::Path, http::StatusCode};

    pub async fn get_chats(user: AuthenticatedUser) -> Result<Json<Vec<Chat>>, StatusCode> {
        Ok(Json(
            db::user_get_chats(user.id).await.map_err(error::db_err)?,
        ))
    }

    pub async fn create_chat(
        user: AuthenticatedUser,
        Json(body): Json<CreateChatRequest>,
    ) -> Result<Json<Chat>, StatusCode> {
        let chat_id: i64 = db::chat_create(&body.chat_name)
            .await
            .map_err(error::db_err)?;
        db::chat_add_user(chat_id, user.id)
            .await
            .map_err(error::db_err)?;
        Ok(Json(db::chat_get(chat_id).await.map_err(error::db_err)?))
    }

    pub async fn get_chat(Path(id): Path<i64>) -> Result<Json<Chat>, StatusCode> {
        Ok(Json(db::chat_get(id).await.map_err(error::db_err)?))
    }

    pub async fn delete_chat(
        Path(id): Path<i64>,
        user: AuthenticatedUser,
    ) -> Result<StatusCode, StatusCode> {
        if db::is_user_in_chat(id, user.id)
            .await
            .map_err(error::db_err)?
        {
            db::chat_delete(id).await.map_err(error::db_err)?;
            Ok(StatusCode::OK)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }

    pub async fn get_chat_members(
        user: AuthenticatedUser,
        Path(id): Path<i64>,
    ) -> Result<Json<Vec<User>>, StatusCode> {
        if db::is_user_in_chat(id, user.id)
            .await
            .map_err(error::db_err)?
        {
            Ok(Json(db::chat_get_members(id).await.map_err(error::db_err)?))
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
    pub async fn add_chat_member(
        user: AuthenticatedUser,
        Path((chat_id, user_id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, StatusCode> {
        if db::is_user_in_chat(chat_id, user.id)
            .await
            .map_err(error::db_err)?
        {
            db::chat_add_user(chat_id, user_id)
                .await
                .map_err(error::db_err)?;
            Ok(StatusCode::OK)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
    pub async fn is_user_in_chat(
        user: AuthenticatedUser,
        Path((chat_id, user_id)): Path<(i64, i64)>,
    ) -> Result<Json<bool>, StatusCode> {
        if db::is_user_in_chat(chat_id, user.id)
            .await
            .map_err(error::db_err)?
        {
            Ok(Json(
                db::is_user_in_chat(chat_id, user_id)
                    .await
                    .map_err(error::db_err)?,
            ))
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
    pub async fn remove_chat_member(
        user: AuthenticatedUser,
        Path((chat_id, user_id)): Path<(i64, i64)>,
    ) -> Result<StatusCode, StatusCode> {
        if db::is_user_in_chat(chat_id, user.id)
            .await
            .map_err(error::db_err)?
        {
            if db::is_user_in_chat(chat_id, user_id)
                .await
                .map_err(error::db_err)?
            {
                db::chat_delete_user(chat_id, user_id)
                    .await
                    .map_err(error::db_err)?;
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
        db, error,
        models::{AuthenticatedUser, GetChatMessagesRequest, Message, SendMessageRequest},
    };
    use axum::{Json, extract::Path, http::StatusCode};

    pub async fn save_message(
        user: AuthenticatedUser,
        Path(chat_id): Path<i64>,
        Json(body): Json<SendMessageRequest>,
    ) -> Result<Json<Message>, StatusCode> {
        if db::is_user_in_chat(chat_id, user.id)
            .await
            .map_err(error::db_err)?
        {
            Ok(Json(
                db::get_message(
                    db::save_message(user.id, chat_id, &body.content)
                        .await
                        .map_err(error::db_err)?,
                )
                .await
                .map_err(error::db_err)?,
            ))
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }

    pub async fn delete_message(
        user: AuthenticatedUser,
        Path(message_id): Path<i64>,
    ) -> Result<StatusCode, StatusCode> {
        if db::get_message(message_id)
            .await
            .map_err(error::db_err)?
            .owner_id
            == user.id
        {
            db::delete_message(message_id)
                .await
                .map_err(error::db_err)?;
            Ok(StatusCode::OK)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }

    pub async fn get_message(
        user: AuthenticatedUser,
        Path(message_id): Path<i64>,
    ) -> Result<Json<Message>, StatusCode> {
        let message = db::get_message(message_id).await.map_err(error::db_err)?;
        if db::is_user_in_chat(message.chat_id, user.id)
            .await
            .map_err(error::db_err)?
        {
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
        let messages = db::chat_get_messages(chat_id, body.limit, body.offset)
            .await
            .map_err(error::db_err)?;
        if db::is_user_in_chat(chat_id, user.id)
            .await
            .map_err(error::db_err)?
        {
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
