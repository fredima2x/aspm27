// Global Imports
use axum::{Router, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};

// Configuration
const SERVER_ADDRESS: &'static str = "127.0.0.1:3000";
const SQLITE_DB_ADDRESS: &'static str = "sqlite:db.sqlite3";
const TOKEN_SECRET: &'static str = "ub+MdZ4ieRpxtlYEZghNhg";

// Data Structures
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
struct User {
    id: i64,
    username: String,
    password_hash: String,
}

// Request Structures
#[derive(Deserialize)]
struct SendUserRequest {
    username: String,
    password: String,
}
#[derive(Serialize)]
struct CreateUserResponse {
    id: i64,
}
#[derive(Serialize)]
struct LoginResponse {
    token_string: String,
}

// Extractors
pub struct AuthenticatedUser {
    pub id: i64,
}
//impl<S> axum::extract::FromRequestParts<S> for AuthenticatedUser
//where
//    S: Send + Sync,
//{
//    type Rejection = StatusCode;

//    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
// dein Code kommt hier rein
//    }
//}

mod auth {
    use argon2::{
        Argon2, PasswordHash, PasswordVerifier,
        password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
    };
    use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
    use serde::{Deserialize, Serialize};

    use crate::TOKEN_SECRET;

    #[derive(Serialize, Deserialize)]
    pub struct Claims {
        pub sub: i64,
        pub exp: usize,
    }

    pub fn hash_password(password: &str) -> String {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .unwrap()
            .to_string()
    }
    pub fn verify_password(password: &str, password_hash: &str) -> bool {
        let parsed_hash = PasswordHash::new(password_hash).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
    pub fn create_token(user_id: i64) -> String {
        let exp_time = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .unwrap()
            .timestamp() as usize;
        let claims = Claims {
            sub: user_id,
            exp: exp_time,
        };
        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(TOKEN_SECRET.as_bytes().as_ref()),
        )
        .unwrap()
    }
    pub fn verify_token(token_string: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = jsonwebtoken::decode::<Claims>(
            &token_string,
            &DecodingKey::from_secret(TOKEN_SECRET.as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}

// Database Handler
mod db {

    use crate::{SQLITE_DB_ADDRESS, User, auth::hash_password};

    use sqlx::SqlitePool;

    pub async fn setup() {
        let pool = get_pool().await;
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id       INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();
    }
    async fn get_pool() -> SqlitePool {
        let pool = SqlitePool::connect(SQLITE_DB_ADDRESS).await.unwrap();
        return pool;
    }

    pub async fn get_users() -> Vec<User> {
        let pool = get_pool().await;
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&pool)
            .await
            .expect("Could not Fetch Users from Database!");
        users
    }

    pub async fn create_user(username: &str, password: &str) -> i64 {
        let pool = get_pool().await;
        let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
            .bind(username)
            .bind(hash_password(password))
            .execute(&pool)
            .await
            .expect("Failed to Create User in Databse!");
        let user_id: i64 = result.last_insert_rowid();
        user_id
    }

    pub async fn delete_user(id: i64) {
        let pool = get_pool().await;
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await
            .expect("Failed to Delete User!");
    }

    pub async fn get_user_by_id(id: i64) -> User {
        let pool = get_pool().await;
        let result: User = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .expect("Failed to get User!");
        result
    }
    pub async fn get_user_by_username(username: &str) -> User {
        let pool = get_pool().await;
        let result: User = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(&pool)
            .await
            .expect("Failed to get User!");
        result
    }
}

// Request Handler
mod handler {
    use crate::{CreateUserResponse, LoginResponse, SendUserRequest, User, auth, db};
    use axum::{Json, extract::Path, http::StatusCode};

    pub async fn get_users() -> Json<Vec<User>> {
        let users = db::get_users().await;
        Json(users)
    }
    pub async fn create_user(Json(body): Json<SendUserRequest>) -> Json<CreateUserResponse> {
        let id: i64 = db::create_user(&body.username, &body.password).await;
        let response = CreateUserResponse { id: id };
        Json(response)
    }
    pub async fn delete_user(Path(id): Path<i64>) {
        db::delete_user(id).await;
    }
    pub async fn get_user(Path(id): Path<i64>) -> Json<User> {
        let user: User = db::get_user_by_id(id).await;
        Json(user)
    }
    pub async fn login(
        Json(body): Json<SendUserRequest>,
    ) -> Result<Json<LoginResponse>, StatusCode> {
        let user: User = db::get_user_by_username(&body.username).await;
        let result: bool = auth::verify_password(&body.password, &user.password_hash);
        if result {
            return Ok(Json(LoginResponse {
                token_string: auth::create_token(user.id),
            }));
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
}

// Entry Point
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users", get(handler::get_users).post(handler::create_user))
        .route(
            "/users/{id}",
            get(handler::get_user).delete(handler::delete_user),
        )
        .route("/login", get(handler::login));
    let listener = tokio::net::TcpListener::bind(SERVER_ADDRESS).await.unwrap();

    db::setup().await;

    axum::serve(listener, app).await.unwrap();
}
