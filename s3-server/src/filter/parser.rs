use axum::async_trait;
use s3_core::S3Error;

use crate::router::Router;

use super::{Filter, S3Data};

pub struct ParserFilter {
    router: Router,
}

impl ParserFilter {
    pub fn new(hosts: Vec<String>) -> Self {
        Self {
            router: Router::new(hosts),
        }
    }
}

#[async_trait]
impl Filter for ParserFilter {
    async fn handle(&mut self, data: &mut S3Data) -> Result<(), S3Error> {
        let result = self.router.match_result(&data.req);

        if !result.key.is_empty() {
            if result.key.len() > 1024 {
                return Err(S3Error::KeyTooLong(result.key));
            }
        }

        if !result.bucket.is_empty() && !s3_core::is_valid_bucket_name(&result.bucket) {
            return Err(S3Error::InvalidBucketName(result.bucket));
        }

        data.action = result.action;
        data.bucket_name = result.bucket;
        data.key = result.key;
        data.host = result.host;
        Ok(())
    }
}
