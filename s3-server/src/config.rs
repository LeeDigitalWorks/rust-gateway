use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub region: String,
    pub s3domain: Vec<String>,
    pub log_path: String,
    pub log_level: String,
    pub debug_mode: bool,
    pub bind_api_address: String,
    pub bind_admin_http_address: String,
    pub bind_admin_grpc_address: String,
    pub iam_address: String,
    pub meta_store: String,
    pub postgresdb_info: Option<String>,
    pub redis_address: String,
    pub redis_username: String,
    pub redis_password: String,
    pub redis_connect_timeout: u64,
    pub redis_read_timeout: u64,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = std::fs::read_to_string(path)?;
        let mut config = toml::from_str(&contents)?;
        config = Config::env_override(config)?;
        Ok(config)
    }

    fn env_override(mut config: Config) -> Result<Self, Box<dyn Error>> {
        if let Ok(postgresdb_info) = std::env::var("DATABASE_URL") {
            config.postgresdb_info = Some(postgresdb_info);
        }
        Ok(config)
    }
}
