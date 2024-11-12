use axum::async_trait;
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
        let mut ip = None;

        if let Some(forwarded_for) = data.req.headers().get("x-forwarded-for") {
            ip = forwarded_for.to_str().ok();
        }
        if let Some(real_ip) = data.req.headers().get("x-real-ip") {
            ip = real_ip.to_str().ok();
        }
        if let Some(cf_ip) = data.req.headers().get("cf-connecting-ip") {
            ip = cf_ip.to_str().ok();
        }
        if ip.is_none() {
            return Err(S3Error::InvalidRequest);
        }

        match self.local_rate_limiter.check_key(&ip.unwrap().to_string()) {
            Ok(_) => {}
            Err(_) => {
                return Err(S3Error::SlowDown);
            }
        }

        // TODO: Implement rate limiting via redis and possibly tokio timeout
        Ok(())
    }
}
