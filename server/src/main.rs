use axum::{
    Router,
    routing::{delete, get, post, put},
};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
    password: String,
}

mod handler {
    use crate::User;
    use axum::{Json, extract::Path};

    pub async fn get_users() -> Json<Vec<User>> {
        let users = vec![User {
            name: "fredima2x".to_string(),
            password: "1234".to_string(),
        }];
        Json(users)
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/users", get(handler::get_users));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
