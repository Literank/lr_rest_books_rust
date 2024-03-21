use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub app: ApplicationConfig,
    pub cache: CacheConfig,
    pub db: DBConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DBConfig {
    pub file_name: String,
    pub dsn: String,
    pub mongo_uri: String,
    pub mongo_db_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CacheConfig {
    pub redis_uri: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationConfig {
    pub address: String,
    pub port: i32,
    pub page_size: u32,
    pub token_secret: String,
    pub token_hours: u32,
}

pub fn parse_config(file_name: &str) -> Config {
    let mut file = File::open(file_name).expect("Failed to open TOML config file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read TOML config file");
    toml::from_str(&content).expect("Failed to parse TOML config file")
}
