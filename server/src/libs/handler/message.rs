use crate::libs::{
    db, error,
    models::{
        api::requests::{GetChatMessagesRequest, SendMessageRequest},
        db_objects::Message,
        misc::AuthenticatedUser,
    },
};
use axum::{Json, extract::Path, http::StatusCode};

pub async fn save_message(
    user: AuthenticatedUser,
    Path(chat_id): Path<i64>,
    Json(body): Json<SendMessageRequest>,
) -> Result<Json<Message>, StatusCode> {
    if db::chats::is_user_in_chat(chat_id, user.id)
        .await
        .map_err(error::db_err)?
    {
        Ok(Json(
            db::message::get_message(
                db::message::save_message(user.id, chat_id, &body.content)
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
    if db::message::get_message(message_id)
        .await
        .map_err(error::db_err)?
        .owner_id
        == user.id
    {
        db::message::delete_message(message_id)
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
    let message = db::message::get_message(message_id)
        .await
        .map_err(error::db_err)?;
    if db::chats::is_user_in_chat(message.chat_id, user.id)
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
    let messages = db::message::chat_get_messages(chat_id, body.limit, body.offset)
        .await
        .map_err(error::db_err)?;
    if db::chats::is_user_in_chat(chat_id, user.id)
        .await
        .map_err(error::db_err)?
    {
        Ok(Json(messages))
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
