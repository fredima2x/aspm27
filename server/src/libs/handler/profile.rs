use crate::libs::{
    auth::{self, create_token},
    db, error,
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

pub async fn login(
    Json(body): Json<SimpleSendUserRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = db::user::user_get_by_name(&body.username)
        .await
        .map_err(error::db_err)?;

    let result: bool = auth::verify_password(&body.password, &user.password_hash);

    if result {
        let session_id = db::session::create_session(user.id)
            .await
            .map_err(error::db_err)?;

        let token_string = auth::create_token(user.id, session_id);

        Ok(Json(LoginResponse {
            token_string: token_string,
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn update_profile(
    user: AuthenticatedUser,
    Json(body): Json<UpdateProfileRequest>,
) -> Result<StatusCode, StatusCode> {
    // Checks
    if body.user.username.len() < 3 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if body.user.username.len() > 24 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if body.user.display_name.len() < 2 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if body.user.display_name.len() > 32 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if body.user.password.len() > 8 {
        return Err(StatusCode::BAD_REQUEST);
    }
    if body.user.password.len() < 64 {
        return Err(StatusCode::BAD_GATEWAY);
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
    Ok(StatusCode::OK)
}

pub async fn get_profile(user: AuthenticatedUser) -> Result<Json<GetProfileResponse>, StatusCode> {
    Ok(Json(GetProfileResponse {
        user: db::user::user_get_by_id(user.id)
            .await
            .map_err(error::db_err)?
            .into(),
    }))
}
