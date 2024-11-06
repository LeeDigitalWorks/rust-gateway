use async_trait::async_trait;
use s3_core::{
    response::ListBucketsResponse,
    types::{BucketContainer, Owner},
    S3Error,
};

pub struct ProxyBackend {}

#[async_trait]
impl crate::Backend for ProxyBackend {
    async fn put_object(&self, _bucket_name: &str, _key: &str, _data: Vec<u8>) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn get_object(&self, _bucket_name: &str, _key: &str) -> Result<Vec<u8>, S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn list_buckets(&self) -> Result<ListBucketsResponse, S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn create_bucket(&self, _bucket_name: &str) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn delete_bucket(&self, _bucket_name: &str) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }
}
