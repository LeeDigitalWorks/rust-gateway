use async_trait::async_trait;
use s3_core::{
    response::ListBucketsResponse,
    types::{BucketContainer, Owner},
    S3Error,
};

pub struct ProxyBackend {}

#[async_trait]
impl crate::Backend for ProxyBackend {
    async fn list_buckets(&self) -> Result<ListBucketsResponse, S3Error> {
        Ok(ListBucketsResponse {
            buckets: BucketContainer { buckets: vec![] },
            owner: Owner {
                id: "".to_string(),
                display_name: "".to_string(),
            },
        })
    }

    async fn create_bucket(&self, _bucket_name: &str) -> Result<(), S3Error> {
        Ok(())
    }

    async fn delete_bucket(&self, _bucket_name: &str) -> Result<(), S3Error> {
        Ok(())
    }
}
