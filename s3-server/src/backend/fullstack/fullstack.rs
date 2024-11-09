use axum::async_trait;
use s3_core::{
    response::ListBucketsResponse,
    types::{BucketContainer, Owner},
    S3Error,
};
use uuid::{timestamp::context, Timestamp, Uuid};

use crate::backend::{
    db::{Database, DatabaseStore},
    types::Bucket,
    StorageBackend,
};

pub struct FullstackBackend {
    postgres: Database,
    proxy: StorageBackend,
}

impl FullstackBackend {
    pub fn new(postgres: Database, proxy: StorageBackend) -> Self {
        Self { postgres, proxy }
    }
}

#[async_trait]
impl crate::backend::Indexer for FullstackBackend {}

#[async_trait]
impl crate::backend::IndexReader for FullstackBackend {
    async fn list_buckets(&self, user_id: &i64) -> Result<ListBucketsResponse, S3Error> {
        let buckets = self
            .postgres
            .list_buckets(user_id)
            .await
            .map_err(|_| S3Error::InternalError)?;
        Ok(ListBucketsResponse {
            buckets: BucketContainer {
                buckets: buckets
                    .iter()
                    .map(|bucket| s3_core::Bucket {
                        name: bucket.name.clone(),
                        creation_date: bucket.created_at.to_string(),
                    })
                    .collect(),
            },
            owner: Owner {
                id: user_id.to_string(),
                display_name: user_id.to_string(),
            },
        })
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
        // Call database backend to create bucket
        self.postgres
            .create_bucket(Bucket {
                id: Uuid::new_v7(Timestamp::now(context::NoContext)),
                name: bucket_name.to_string(),
                user_id: *user_id,
                created_at: chrono::Utc::now(),
            })
            .await
            .map_err(|e| {
                tracing::error!("Error creating bucket: {:?}", e);
                S3Error::InternalError
            })?;
        // Call proxy backend to create bucket
        self.proxy.create_bucket(bucket_name, user_id).await?;
        Ok(())
    }

    async fn delete_bucket(&self, bucket_name: &str, user_id: &i64) -> Result<(), S3Error> {
        // Delete bucket from proxy backend
        self.proxy.delete_bucket(bucket_name, user_id).await?;
        // Delete bucket from database backend
        self.postgres
            .delete_bucket(bucket_name, user_id)
            .await
            .map_err(|e| {
                tracing::error!("Error deleting bucket: {:?}", e);
                S3Error::InternalError
            })?;
        Ok(())
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
