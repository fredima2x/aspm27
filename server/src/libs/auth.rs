use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub session_id: i64,
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

pub fn create_token(user_id: Uuid, session_id: i64, secret: &str) -> String {
    let exp_time = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;
    let claims = Claims {
        sub: user_id,
        session_id: session_id,
        exp: exp_time,
    };
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

pub fn verify_token(
    token_string: &str,
    secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = jsonwebtoken::decode::<Claims>(
        token_string,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
