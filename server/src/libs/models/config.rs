use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub jwt_signing_secret: String,
    pub host_address: String,
    pub database_url: String,
}
