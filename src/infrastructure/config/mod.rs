use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub app: ApplicationConfig,
    pub db: DBConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DBConfig {
    pub file_name: String,
    pub dsn: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationConfig {
    pub port: i32,
}
