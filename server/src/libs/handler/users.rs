use crate::libs::{
    check, db, error,
    models::{
        api::{requests::SimpleSendUserRequest, responses::CreateUserResponse},
        db_objects::BasicUser,
        misc::AuthenticatedUser,
    },
};
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;

// DEBUG ONLY REMOVE IN PRODUCTION
// pub async fn get_users() -> Result<Json<Vec<DirectUser>>, StatusCode> {
//     let users = db::user::user_getall().await.map_err(error::db_err)?;
//     Ok(Json(users))
// }

#[tracing::instrument]
pub async fn create_user(
    Json(body): Json<SimpleSendUserRequest>,
) -> Result<Json<CreateUserResponse>, StatusCode> {
    tracing::info!("Creating user");

    let (username_valid, password_valid) = tokio::join!(
        check::check_password(&body.password),
        check::check_username(&body.username),
    );

    if !username_valid || !password_valid {
        tracing::info!("Returning BAD_REQUEST due to invalid username or password");
        return Err(StatusCode::BAD_REQUEST);
    }

    let id = db::user::user_create(&body.username, &body.password)
        .await
        .map_err(error::db_err)?;

    tracing::info!("User created successfully");
    Ok(Json(CreateUserResponse { id }))
}

#[tracing::instrument]
pub async fn delete_user(user: AuthenticatedUser) -> Result<StatusCode, StatusCode> {
    tracing::info!("Deleting user");
    db::user::user_soft_delete(user.id)
        .await
        .map_err(error::db_err)?;
    tracing::info!("User deleted successfully");
    Ok(StatusCode::OK)
}

#[tracing::instrument]
pub async fn get_user(Path(id): Path<i64>) -> Result<Json<BasicUser>, StatusCode> {
    tracing::info!("Getting user");
    let user: BasicUser = db::user::user_get_by_id(id)
        .await
        .map_err(error::db_err)?
        .into();
    tracing::info!("User retrieved successfully");
    Ok(Json(user))
}
