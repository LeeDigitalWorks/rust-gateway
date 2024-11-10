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
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&contents)?)
    }
}
