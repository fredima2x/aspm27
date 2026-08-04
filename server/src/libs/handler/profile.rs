use crate::libs::{
    auth, db, error,
    models::{
        api::{
            requests::{SimpleSendUserRequest, UpdateProfileRequest},
            responses::{GetProfileResponse, LoginResponse},
        },
        db_objects::User,
        misc::AuthenticatedUser,
    },
};
use axum::Json;
use axum::http::StatusCode;

pub async fn login(
    Json(body): Json<SimpleSendUserRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user: User = db::user::user_get_by_name(&body.username)
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
    db::user::update_user(User {
        id: user.id,
        username: body.user.username,
        display_name: body.user.display_name,
        password_hash: auth::hash_password(&body.user.password),
    })
    .await
    .map_err(error::db_err)?;
    Ok(StatusCode::OK)
}

pub async fn get_profile(user: AuthenticatedUser) -> Result<Json<GetProfileResponse>, StatusCode> {
    Ok(Json(GetProfileResponse {
        user: db::user::user_get_by_id(user.id)
            .await
            .map_err(error::db_err)?,
    }))
}
