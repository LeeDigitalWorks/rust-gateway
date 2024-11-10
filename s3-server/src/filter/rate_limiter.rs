use axum::async_trait;
use s3_core::S3Error;

use super::{Filter, S3Data};

pub struct RateLimitFilter {
    redis_client: redis::Client,
}

impl RateLimitFilter {
    pub fn new(redis_client: redis::Client) -> Self {
        Self { redis_client }
    }
}

#[async_trait]
impl Filter for RateLimitFilter {
    async fn handle(&self, data: &mut S3Data) -> Result<(), S3Error> {
        // Use local rate limiter first

        // TODO: Implement rate limiting via redis and possibly tokio timeout
        Ok(())
    }
}
