use std::net::SocketAddr;

use axum::{async_trait, extract::ConnectInfo};
use s3_core::S3Error;

use super::{Filter, S3Data};

pub struct RateLimitFilter {
    redis_client: redis::cluster::ClusterClient,
    local_rate_limiter: governor::DefaultKeyedRateLimiter<String>,
}

impl RateLimitFilter {
    pub fn new(
        redis_client: redis::cluster::ClusterClient,
        local_rate_limiter: governor::DefaultKeyedRateLimiter<String>,
    ) -> Self {
        Self {
            redis_client,
            local_rate_limiter,
        }
    }
}

#[async_trait]
impl Filter for RateLimitFilter {
    async fn handle(&self, data: &mut S3Data) -> Result<(), S3Error> {
        // Take token from both local and redis rate limiters
        // Use local rate limiter first
        let ip = data
            .req
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .ok_or(S3Error::AccessDenied)?
            .to_string();

        match self.local_rate_limiter.check_key(&ip) {
            Ok(_) => {}
            Err(_) => {
                return Err(S3Error::SlowDown);
            }
        }

        // TODO: Implement rate limiting via redis and possibly tokio timeout
        Ok(())
    }
}
