use crate::libs::models::db_objects::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i64,
}

#[derive(Serialize)]
pub struct GetProfileResponse {
    pub user: User,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token_string: String,
}
