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
        data.operation = self.router.match_result(&data.req);
        Ok(())
    }
}
