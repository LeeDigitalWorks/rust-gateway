use std::sync::Arc;

use axum::async_trait;
use s3_core::S3Error;

use crate::backend::Indexer;

use super::{Filter, S3Data};

pub struct BucketFilter {
    indexer: Arc<Box<dyn Indexer>>,
}

impl BucketFilter {
    pub fn new(indexer: Arc<Box<dyn Indexer>>) -> Self {
        Self { indexer }
    }
}

#[async_trait]
impl Filter for BucketFilter {
    async fn handle(&self, data: &mut S3Data) -> Result<(), S3Error> {
        if data.action != s3_core::S3Action::CreateBucket {
            match self.indexer.get_bucket(&data.bucket_name).await {
                Ok(bucket) => {
                    data.bucket = Some(bucket);
                }
                Err(_) => {
                    return Err(S3Error::NoSuchBucket(data.bucket_name.clone()));
                }
            }
        }
        Ok(())
    }
}
