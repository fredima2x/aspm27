use crate::libs::models::db_objects::BasicUser;
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i64,
}

#[derive(Serialize)]
pub struct GetProfileResponse {
    pub user: BasicUser,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token_string: String,
    pub session_id: i64,
}
