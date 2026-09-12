use axum::http::StatusCode;
use sqlx::error::ErrorKind;

pub fn db_err(e: sqlx::Error) -> StatusCode {
    tracing::error!("Datenbankfehler: {}", e);
    match e {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        sqlx::Error::Database(db_err) if db_err.kind() == ErrorKind::UniqueViolation => {
            StatusCode::CONFLICT
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[allow(dead_code)]
pub fn internal_error<E: std::fmt::Display>(e: E) -> StatusCode {
    tracing::error!("Internal Error: {}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}
