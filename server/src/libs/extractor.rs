use crate::AppState;
use crate::libs::auth;
use crate::libs::models::misc::AuthenticatedUser;
use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};

impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = match parts.headers.get("Authorization") {
            Some(header) => header,
            None => {
                tracing::debug!(
                    "Authentication failed: missing Authorization header"
                );
                return Err(StatusCode::UNAUTHORIZED);
            }
        };

        let auth_str = match auth_header.to_str() {
            Ok(value) => value,
            Err(error) => {
                tracing::debug!(
                    error = %error,
                    "Authentication failed: invalid Authorization header"
                );
                return Err(StatusCode::UNAUTHORIZED);
            }
        };

        let token = match auth_str.strip_prefix("Bearer ") {
            Some(token) if !token.is_empty() => token,
            _ => {
                tracing::debug!(
                    "Authentication failed: invalid Bearer authorization"
                );
                return Err(StatusCode::UNAUTHORIZED);
            }
        };

        let claims = match auth::verify_token(
            token,
            &state.config.jwt_signing_secret,
            state.db.clone(),
        )
        .await
        {
            Ok(claims) => claims,

            Err(auth::VerifyTokenError::InvalidSession) => {
                tracing::debug!(
                    "Authentication failed: invalid session"
                );
                return Err(StatusCode::UNAUTHORIZED);
            }

            Err(auth::VerifyTokenError::Jwt(error)) => {
                tracing::debug!(
                    error = %error,
                    "Authentication failed: invalid JWT"
                );
                return Err(StatusCode::UNAUTHORIZED);
            }

            Err(auth::VerifyTokenError::Database(error)) => {
                tracing::error!(
                    error = %error,
                    "Authentication failed: database error"
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        tracing::debug!(
            user_id = %claims.sub,
            session_id = %claims.session_id,
            "Authentication successful"
        );

        Ok(AuthenticatedUser {
            id: claims.sub,
            session_id: claims.session_id,
        })
    }
}