use axum::http::StatusCode;

pub fn db_err(e: sqlx::Error) -> StatusCode {
    tracing::error!("Datenbankfehler: {}", e);
    match e {
        sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
