use crate::libs::{
    db, error,
    models::{
        api::{requests::SimpleSendUserRequest, responses::CreateUserResponse},
        db_objects::User,
        misc::AuthenticatedUser,
    },
};
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;

pub async fn get_users() -> Result<Json<Vec<User>>, StatusCode> {
    let users = db::user::user_getall().await.map_err(error::db_err)?;
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

    let id = db::user::user_create(&body.username, &body.password)
        .await
        .map_err(error::db_err)?;
    Ok(Json(CreateUserResponse { id }))
}

pub async fn delete_user(user: AuthenticatedUser) -> Result<StatusCode, StatusCode> {
    db::user::user_delete(user.id)
        .await
        .map_err(error::db_err)?;
    Ok(StatusCode::OK)
}

pub async fn get_user(Path(id): Path<i64>) -> Result<Json<User>, StatusCode> {
    let user: User = db::user::user_get_by_id(id).await.map_err(error::db_err)?;
    Ok(Json(user))
}
