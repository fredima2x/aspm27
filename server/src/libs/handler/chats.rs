use crate::libs::{
    db, error,
    models::{
        api::requests::{CreateChatRequest, UpdateChatRequest},
        db_objects::{Chat, User},
        misc::AuthenticatedUser,
    },
};
use axum::{Json, extract::Path, http::StatusCode};

pub async fn get_chats(user: AuthenticatedUser) -> Result<Json<Vec<Chat>>, StatusCode> {
    Ok(Json(
        db::chats::user_get_chats(user.id)
            .await
            .map_err(error::db_err)?,
    ))
}

pub async fn create_chat(
    user: AuthenticatedUser,
    Json(body): Json<CreateChatRequest>,
) -> Result<Json<Chat>, StatusCode> {
    let chat_id: i64 = db::chats::chat_create(&body.chat_name)
        .await
        .map_err(error::db_err)?;
    db::chats::chat_add_user(chat_id, user.id)
        .await
        .map_err(error::db_err)?;
    Ok(Json(
        db::chats::chat_get(chat_id).await.map_err(error::db_err)?,
    ))
}

pub async fn get_chat(Path(id): Path<i64>) -> Result<Json<Chat>, StatusCode> {
    Ok(Json(db::chats::chat_get(id).await.map_err(error::db_err)?))
}

pub async fn delete_chat(
    Path(id): Path<i64>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    if db::chats::is_user_in_chat(id, user.id)
        .await
        .map_err(error::db_err)?
    {
        db::chats::chat_delete(id).await.map_err(error::db_err)?;
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn get_chat_members(
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<Json<Vec<User>>, StatusCode> {
    if db::chats::is_user_in_chat(id, user.id)
        .await
        .map_err(error::db_err)?
    {
        Ok(Json(
            db::chats::chat_get_members(id)
                .await
                .map_err(error::db_err)?,
        ))
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
pub async fn add_chat_member(
    user: AuthenticatedUser,
    Path((chat_id, user_id)): Path<(i64, i64)>,
) -> Result<StatusCode, StatusCode> {
    if db::chats::is_user_in_chat(chat_id, user.id)
        .await
        .map_err(error::db_err)?
    {
        db::chats::chat_add_user(chat_id, user_id)
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
    if db::chats::is_user_in_chat(chat_id, user.id)
        .await
        .map_err(error::db_err)?
    {
        Ok(Json(
            db::chats::is_user_in_chat(chat_id, user_id)
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
    if db::chats::is_user_in_chat(chat_id, user.id)
        .await
        .map_err(error::db_err)?
    {
        if db::chats::is_user_in_chat(chat_id, user_id)
            .await
            .map_err(error::db_err)?
        {
            db::chats::chat_delete_user(chat_id, user_id)
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

pub async fn update_chat(
    user: AuthenticatedUser,
    Path(id): Path<i64>,
    Json(body): Json<UpdateChatRequest>,
) -> Result<StatusCode, StatusCode> {
    if db::chats::is_user_in_chat(id, user.id)
        .await
        .map_err(error::db_err)?
    {
        db::chats::update_chat(Chat {
            id,
            chat_name: body.chat_name,
            chat_desc: body.chat_desc,
        })
        .await
        .map_err(error::db_err)?;
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
