use crate::libs::{
    check, db, error,
    models::{
        api::{requests::SimpleSendUserRequest, responses::CreateUserResponse},
        app_state::AppState,
        db_objects::BasicUser,
        misc::AuthenticatedUser,
    },
};
use axum::Json;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;

#[tracing::instrument]
pub async fn create_user(
    State(state): State<AppState>,
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

    let id = db::user::user_create(&body.username, &body.password, state.db.clone())
        .await
        .map_err(error::db_err)?;

    tracing::info!("User created successfully");
    Ok(Json(CreateUserResponse { id }))
}

#[tracing::instrument]
pub async fn delete_user(
    user: AuthenticatedUser,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("Deleting user");
    db::user::user_soft_delete(user.id, state.db.clone())
        .await
        .map_err(error::db_err)?;
    tracing::info!("User deleted successfully");
    Ok(StatusCode::OK)
}

#[tracing::instrument]
pub async fn get_user_by_id(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<Json<BasicUser>, StatusCode> {
    tracing::info!("Getting user by id");
    let user: BasicUser = db::user::user_get_by_id(id, state.db.clone())
        .await
        .map_err(error::db_err)?
        .into();
    tracing::info!("User retrieved successfully");
    Ok(Json(user))
}

#[tracing::instrument]
pub async fn get_user_by_name(
    Path(name): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BasicUser>, StatusCode> {
    tracing::info!("Getting user by name");
    let user: BasicUser = db::user::user_get_by_name(&name, state.db.clone())
        .await
        .map_err(error::db_err)?
        .into();
    tracing::info!("User retrieved succesfully");
    Ok(Json(user))
}
