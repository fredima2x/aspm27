use axum::http::StatusCode;

pub fn db_err(e: sqlx::Error) -> StatusCode {
    tracing::error!("Datenbankfehler: {}", e);
    StatusCode::INTERNAL_SERVER_ERROR
}
