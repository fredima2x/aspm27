use crate::libs::{
    db, error,
    models::{
        api::requests::{CreateChatRequest, UpdateChatRequest},
        db_objects::{BasicChat, BasicUser},
        misc::AuthenticatedUser,
    },
};
use axum::{Json, extract::Path, http::StatusCode};
use tracing::instrument;

pub async fn get_chats(user: AuthenticatedUser) -> Result<Json<Vec<BasicChat>>, StatusCode> {
    tracing::info!("Got get_chats Request.");
    Ok(Json(
        db::chats::user_get_chats(user.id)
            .await
            .map_err(error::db_err)?
            .into_iter()
            .map(|c| c.into())
            .collect(),
    ))
}

#[instrument]
pub async fn create_chat(
    user: AuthenticatedUser,
    Json(body): Json<CreateChatRequest>,
) -> Result<Json<BasicChat>, StatusCode> {
    tracing::info!("Got create_chats Request.");
    // Checks
    if body.chat_name.len() > 24 {
        tracing::debug!("Returned BAD_REQUEST because chat_name was longer than 24 chars.");
        return Err(StatusCode::BAD_REQUEST);
    }
    if body.chat_name.len() < 3 {
        tracing::debug!("Returned BAD_REQUEST because chat_name was shorter than 3 chars.");
        return Err(StatusCode::BAD_REQUEST);
    }

    let chat_id: i64 = db::chats::chat_create(&body.chat_name)
        .await
        .map_err(error::db_err)?;
    db::chats::chat_add_user(chat_id, user.id)
        .await
        .map_err(error::db_err)?;
    tracing::debug!("Succesfully Created Chat");
    Ok(Json(
        db::chats::chat_get(chat_id)
            .await
            .map_err(error::db_err)?
            .into(),
    ))
}

pub async fn get_chat(Path(id): Path<i64>) -> Result<Json<BasicChat>, StatusCode> {
    tracing::info!("Got get_chat Request.");
    Ok(Json(
        db::chats::chat_get(id).await.map_err(error::db_err)?.into(),
    ))
}

pub async fn delete_chat(
    Path(id): Path<i64>,
    user: AuthenticatedUser,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("Got delete_chats Request.");
    if db::chats::is_user_in_chat(id, user.id)
        .await
        .map_err(error::db_err)?
    {
        db::chats::chat_soft_delete(id)
            .await
            .map_err(error::db_err)?;
        Ok(StatusCode::OK)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

pub async fn get_chat_members(
    user: AuthenticatedUser,
    Path(id): Path<i64>,
) -> Result<Json<Vec<BasicUser>>, StatusCode> {
    tracing::info!("Got get_chat_members Request.");
    if db::chats::is_user_in_chat(id, user.id)
        .await
        .map_err(error::db_err)?
    {
        Ok(Json(
            db::chats::chat_get_members(id)
                .await
                .map_err(error::db_err)?
                .into_iter()
                .map(|c| c.into())
                .collect(),
        ))
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}
pub async fn add_chat_member(
    user: AuthenticatedUser,
    Path((chat_id, user_id)): Path<(i64, i64)>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("Got add_chat_member Request.");
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
    tracing::info!("Got is_user_in_chat Request");
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
    tracing::info!("Got remove_chat_member Request.");
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
    tracing::info!("Got update_chat Request.");
    // Checks
    if body.chat_name.len() > 24 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if body.chat_name.len() < 3 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if body.chat_desc.len() > 255 {
        return Err(StatusCode::BAD_REQUEST);
    }

    if db::chats::is_user_in_chat(id, user.id)
        .await
        .map_err(error::db_err)?
    {
        db::chats::update_chat(BasicChat {
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
