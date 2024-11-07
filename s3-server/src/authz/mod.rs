mod handler;
pub use handler::*;

mod v4;
mod v4_parser;
mod v4_utils;

#[derive(Debug, Clone)]
pub struct Key {
    pub access_key: String,
    pub secret_key: String,
    pub user_id: u64,
}
