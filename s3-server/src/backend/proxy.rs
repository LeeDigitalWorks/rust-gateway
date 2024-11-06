use axum::async_trait;
use s3_core::{
    response::ListBucketsResponse,
    types::{BucketContainer, Owner},
    S3Error,
};

pub struct ProxyBackend {
    pub s3_client: aws_sdk_s3::Client,
}

#[async_trait]
impl crate::backend::Indexer for ProxyBackend {
    async fn put_object(&self, _bucket_name: &str, _key: &str, _data: Vec<u8>) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn get_object(&self, _bucket_name: &str, _key: &str) -> Result<Vec<u8>, S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn list_buckets(&self, _user_id: &u64) -> Result<ListBucketsResponse, S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn create_bucket(&self, _bucket_name: &str, _user_id: &u64) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn delete_bucket(&self, _bucket_name: &str, _user_id: &u64) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }
}
