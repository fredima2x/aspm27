use crate::libs::{
    db, error,
    models::{
        api::requests::{GetChatMessagesRequest, SendMessageRequest},
        db_objects::BasicMessage,
        misc::AuthenticatedUser,
    },
};
use axum::{Json, extract::Path, http::StatusCode};

#[tracing::instrument]
pub async fn save_message(
    user: AuthenticatedUser,
    Path(chat_id): Path<i64>,
    Json(body): Json<SendMessageRequest>,
) -> Result<Json<BasicMessage>, StatusCode> {
    tracing::info!("Got save_message Request.");
    if db::chats::is_user_in_chat(chat_id, user.id)
        .await
        .map_err(error::db_err)?
    {
        tracing::info!("User {} is in chat {}.", user.id, chat_id);
        Ok(Json(
            db::message::get_message(
                db::message::save_message(user.id, chat_id, &body.content)
                    .await
                    .map_err(error::db_err)?,
            )
            .await
            .map_err(error::db_err)?
            .into(),
        ))
    } else {
        tracing::info!("Returned FORBIDDEN because User {} isn't in chat.", user.id);
        Err(StatusCode::FORBIDDEN)
    }
}

#[tracing::instrument]
pub async fn delete_message(
    user: AuthenticatedUser,
    Path(message_id): Path<i64>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("Got delete_message Request.");
    if db::message::get_message(message_id)
        .await
        .map_err(error::db_err)?
        .owner_id
        == user.id
    {
        db::message::message_soft_delete(message_id)
            .await
            .map_err(error::db_err)?;
        tracing::info!("Message {} deleted successfully.", message_id);
        Ok(StatusCode::OK)
    } else {
        tracing::info!(
            "Returned FORBIDDEN because User {} isn't the owner of message {}.",
            user.id,
            message_id
        );
        Err(StatusCode::FORBIDDEN)
    }
}

#[tracing::instrument]
pub async fn get_message(
    user: AuthenticatedUser,
    Path(message_id): Path<i64>,
) -> Result<Json<BasicMessage>, StatusCode> {
    tracing::info!("Got get_message Request.");
    let message = db::message::get_message(message_id)
        .await
        .map_err(error::db_err)?;
    tracing::info!("Message {} retrieved successfully.", message_id);
    if db::chats::is_user_in_chat(message.chat_id, user.id)
        .await
        .map_err(error::db_err)?
    {
        tracing::info!("User {} is in chat {}.", user.id, message.chat_id);
        Ok(Json(message.into()))
    } else {
        tracing::info!(
            "Returned FORBIDDEN because User {} is not in chat {}.",
            user.id,
            message.chat_id
        );
        Err(StatusCode::FORBIDDEN)
    }
}

#[tracing::instrument]
pub async fn get_chat_messages(
    user: AuthenticatedUser,
    Path(chat_id): Path<i64>,
    Json(body): Json<GetChatMessagesRequest>,
) -> Result<Json<Vec<BasicMessage>>, StatusCode> {
    tracing::info!("Got get_chat_messages Request.");
    let messages = db::message::chat_get_messages(chat_id, body.limit, body.offset)
        .await
        .map_err(error::db_err)?;
    if db::chats::is_user_in_chat(chat_id, user.id)
        .await
        .map_err(error::db_err)?
    {
        tracing::info!("User {} is in chat {}.", user.id, chat_id);
        Ok(Json(messages.into_iter().map(|c| c.into()).collect()))
    } else {
        tracing::info!(
            "Returned FORBIDDEN because User {} is not in chat {}.",
            user.id,
            chat_id
        );
        Err(StatusCode::FORBIDDEN)
    }
}
