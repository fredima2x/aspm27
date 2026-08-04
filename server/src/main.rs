use axum::{
    Router,
    routing::{get, post},
};

use tower_http::cors::CorsLayer;

mod libs;
use crate::libs::handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler::misc::hi))
        .route(
            "/chats",
            get(handler::chats::get_chats).post(handler::chats::create_chat),
        )
        .route(
            "/chats/{id}",
            get(handler::chats::get_chat).delete(handler::chats::delete_chat),
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
            get(handler::message::get_chat_messages).post(handler::message::save_message),
        )
        .route(
            "/messages/{message_id}",
            get(handler::message::get_message).delete(handler::message::delete_message),
        )
        .route(
            "/users",
            get(handler::users::get_users).post(handler::users::create_user),
        )
        .route(
            "/users/{id}",
            get(handler::users::get_user).delete(handler::users::delete_user),
        )
        .route(
            "/profile",
            get(handler::profile::get_profile).put(handler::profile::update_profile),
        )
        .route("/login", post(handler::profile::login))
        // Erlaubt deinem Browser-Frontend Zugriffe
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind(libs::config::SERVER_ADDRESS)
        .await
        .unwrap();

    println!("Server läuft auf {}", libs::config::SERVER_ADDRESS);
    tracing_subscriber::fmt::init();
    libs::db::setup::setup().await;
    axum::serve(listener, app).await.unwrap();
}
