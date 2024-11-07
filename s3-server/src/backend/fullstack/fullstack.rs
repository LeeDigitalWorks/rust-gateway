use axum::async_trait;
use s3_core::{
    response::ListBucketsResponse,
    types::{BucketContainer, Owner},
    S3Error,
};

use crate::backend::{db::Database, ProxyBackend};

pub struct FullstackBackend {
    postgres: Database,
    proxy: ProxyBackend,
}

impl FullstackBackend {
    pub fn new(postgres: Database, proxy: ProxyBackend) -> Self {
        Self { postgres, proxy }
    }
}

#[async_trait]
impl crate::backend::Indexer for FullstackBackend {}

#[async_trait]
impl crate::backend::IndexReader for FullstackBackend {
    async fn list_buckets(&self, user_id: &i64) -> Result<ListBucketsResponse, S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn get_object(&self, bucket_name: &str, key: &str) -> Result<Vec<u8>, S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn list_objects(&self, bucket_name: &str) -> Result<Vec<String>, S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn list_object_versions(&self, _bucket_name: &str, _key: &str) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn list_parts(
        &self,
        _bucket_name: &str,
        _key: &str,
        _upload_id: &str,
    ) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }
}

#[async_trait]
impl crate::backend::IndexWriter for FullstackBackend {
    async fn create_bucket(&self, bucket_name: &str, user_id: &i64) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn delete_bucket(&self, bucket_name: &str, user_id: &i64) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn put_object(&self, bucket_name: &str, key: &str, data: Vec<u8>) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn delete_object(&self, bucket_name: &str, key: &str) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }

    async fn delete_objects(&self, bucket_name: &str, keys: Vec<String>) -> Result<(), S3Error> {
        Err(S3Error::NotImplemented)
    }
}
