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
    storage: StorageBackend,
}

impl FullstackBackend {
    pub fn new(postgres: Database, storage: StorageBackend) -> Self {
        Self { postgres, storage }
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
        // Check if user has reached the maximum number of buckets
        let bucket_quota = self
            .postgres
            .get_bucket_quota(user_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => S3Error::AccessDenied,
                _ => {
                    tracing::error!("Error getting bucket quota: {:?}", e);
                    S3Error::InternalError
                }
            })?;
        let buckets = self.postgres.list_buckets(user_id).await.map_err(|e| {
            tracing::error!("Error listing buckets: {:?}", e);
            S3Error::InternalError
        })?;
        if buckets.len() as i64 >= bucket_quota {
            return Err(S3Error::TooManyBuckets);
        }
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
        self.storage.create_bucket(bucket_name, user_id).await?;
        Ok(())
    }

    async fn delete_bucket(&self, bucket_name: &str, user_id: &i64) -> Result<(), S3Error> {
        // Check if bucket is not empty

        // Delete bucket from database backend
        self.postgres
            .delete_bucket(bucket_name, user_id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => S3Error::NoSuchBucket(bucket_name.to_string()),
                _ => {
                    tracing::error!("Error deleting bucket: {:?}", e);
                    S3Error::InternalError
                }
            })?;
        // Delete bucket from proxy backend
        self.storage.delete_bucket(bucket_name, user_id).await?;
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
