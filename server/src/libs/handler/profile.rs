use crate::libs::{
    auth, check, db, error,
    models::{
        api::{
            requests::{SimpleSendUserRequest, UpdateProfileRequest},
            responses::{GetProfileResponse, LoginResponse},
        },
        db_objects::BasicUser,
        misc::AuthenticatedUser,
    },
};
use axum::Json;
use axum::http::StatusCode;

#[tracing::instrument]
pub async fn login(
    Json(body): Json<SimpleSendUserRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    tracing::info!("Got login request for username: {}", body.username);

    let user = db::user::user_get_by_name(&body.username)
        .await
        .map_err(error::db_err)?;

    let result: bool = auth::verify_password(&body.password, &user.password_hash);

    if result {
        let session_id = db::session::create_session(user.id)
            .await
            .map_err(error::db_err)?;

        let token_string = auth::create_token(user.id, session_id);
        tracing::info!("Login successful for user: {}", body.username);
        Ok(Json(LoginResponse {
            token_string: token_string,
        }))
    } else {
        tracing::info!("Login failed for user: {}", body.username);
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[tracing::instrument]
pub async fn update_profile(
    user: AuthenticatedUser,
    Json(body): Json<UpdateProfileRequest>,
) -> Result<StatusCode, StatusCode> {
    tracing::info!("Updating profile for user: {}", user.id);

    let (username_valid, display_name_valid, password_valid) = tokio::join!(
        check::check_username(&body.user.username),
        check::check_display_name(&body.user.display_name),
        check::check_password(&body.user.password),
    );
    if !username_valid || !display_name_valid || !password_valid {
        tracing::info!(
            "Returned BAD_REQUEST because username, display_name, or password is invalid."
        );
        return Err(StatusCode::BAD_REQUEST);
    }

    db::user::update_user(
        BasicUser {
            id: user.id,
            username: body.user.username,
            display_name: body.user.display_name,
        },
        auth::hash_password(&body.user.password),
    )
    .await
    .map_err(error::db_err)?;
    tracing::info!("Updated profile for user: {}", user.id);
    Ok(StatusCode::OK)
}

#[tracing::instrument]
pub async fn get_profile(user: AuthenticatedUser) -> Result<Json<GetProfileResponse>, StatusCode> {
    tracing::info!("Getting profile for user: {}", user.id);
    Ok(Json(GetProfileResponse {
        user: db::user::user_get_by_id(user.id)
            .await
            .map_err(error::db_err)?
            .into(),
    }))
}
