use axum::async_trait;
use s3_core::S3Error;

use super::{Filter, S3Data};

pub struct SecretKeyFilter {}

impl SecretKeyFilter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Filter for SecretKeyFilter {
    async fn handle(&mut self, data: &mut S3Data) -> Result<(), S3Error> {
        // TODO: Verify secret key against cache
        if data.auth_key.secret_key.is_empty() {
            return Err(S3Error::InvalidAccessKeyId);
        }
        Ok(())
    }
}
