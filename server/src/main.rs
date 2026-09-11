use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;
use tracing_subscriber::EnvFilter;

mod libs;
use crate::libs::{config::load_config, handler, models::app_state::AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Loading Config...");
    let config = load_config();
    if config.jwt_signing_secret == "DEFAULT" || config.jwt_signing_secret == "" {
        tracing::error!("Default or Invalid JWT-Secret! PLEASE CHANGE");
        return;
    }

    tracing::info!("Connecting to Database...");
    let db = sqlx::SqlitePool::connect(&config.database_url)
        .await
        .expect("Could not connect to database!");

    let state = AppState { config, db };

    tracing::debug!("Creating APP Router...");
    let app = Router::new()
        .route("/", get(handler::hi::hi))
        .route("/html", get(handler::hi::hiv2))
        .route(
            "/chats",
            get(handler::chats::get_chats).post(handler::chats::create_chat),
        )
        .route(
            "/chats/{id}",
            get(handler::chats::get_chat)
                .delete(handler::chats::delete_chat)
                .put(handler::chats::update_chat),
        )
        .route("/chats/{id}/users", get(handler::chats::get_chat_members))
        .route(
            "/chats/{chat_id}/users/{user_id}",
            get(handler::chats::is_user_in_chat)
                .post(handler::chats::add_chat_member)
                .delete(handler::chats::remove_chat_member),
        )
        .route(
            "/chats/{chat_id}/messages",
            post(handler::message::save_message),
        )
        .route(
            "/chats/{chat_id}/get_messages",
            post(handler::message::get_chat_messages),
        )
        .route(
            "/messages/{message_id}",
            get(handler::message::get_message).delete(handler::message::delete_message),
        )
        .route("/users", post(handler::users::create_user))
        .route(
            "/users/id/{id}",
            get(handler::users::get_user_by_id).delete(handler::users::delete_user),
        )
        .route("/users/name/{name}", get(handler::users::get_user_by_name))
        .route(
            "/profile",
            get(handler::profile::get_profile).put(handler::profile::update_profile),
        )
        .route("/login", post(handler::profile::login))
        // Erlaubt deinem Browser-Frontend Zugriffe
        .layer(CorsLayer::permissive())
        .with_state(state.clone());

    tracing::info!(
        "Binding to Server Address [{}]...",
        state.config.host_address
    );
    let listener = tokio::net::TcpListener::bind(state.config.host_address)
        .await
        .unwrap();

    tracing::info!("Server running...");
    libs::db::setup::setup(state.db.clone()).await;
    axum::serve(listener, app).await.unwrap();
}
