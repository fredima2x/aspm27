use crate::libs::models::config::Config;
use std::fs;

pub fn load_config() -> Config {
    let data = fs::read_to_string("config.json").expect("config.json konnte nicht gelesen werden");
    serde_json::from_str(&data).expect("config.json enthält ungültiges JSON")
}
