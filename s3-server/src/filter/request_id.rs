use axum::async_trait;
use s3_core::S3Error;

use super::{Filter, S3Data};

pub struct RequestIdFilter {}

impl RequestIdFilter {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Filter for RequestIdFilter {
    async fn handle(&self, data: &mut S3Data) -> Result<(), S3Error> {
        data.request_id = uuid::Uuid::new_v4().to_string();
        Ok(())
    }
}
