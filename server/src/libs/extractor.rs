use crate::libs::auth;
use crate::libs::models::misc::AuthenticatedUser;
use axum::extract::FromRequestParts;
use axum::http::{StatusCode, request::Parts};

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let auth_str = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

        let token = auth_str
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let claims = auth::verify_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthenticatedUser {
            id: claims.sub,
            session_id: claims.session_id,
        })
    }
}
