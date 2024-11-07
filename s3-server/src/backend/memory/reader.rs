use axum::async_trait;
use chrono::Timelike;
use s3_core::response::ListBucketsResponse;
use s3_core::types::{BucketContainer, Owner};
use s3_core::S3Error;
use uuid::{Timestamp, Uuid};

use std::collections::HashMap;
use std::sync::RwLock;

use super::super::types::Bucket;

#[async_trait]
impl crate::backend::IndexReader for crate::backend::memory::InMemoryBackend {
    async fn list_buckets(&self, user_id: &u64) -> Result<ListBucketsResponse, S3Error> {
        let buckets = self.owner_buckets.read().unwrap();
        let buckets = buckets
            .get(user_id)
            .unwrap_or(&vec![])
            .to_owned()
            .iter()
            .map(|bucket| s3_core::Bucket {
                name: bucket.name.clone(),
                creation_date: bucket.created_at.to_string().clone(),
            })
            .collect();
        Ok(ListBucketsResponse {
            buckets: BucketContainer { buckets },
            owner: Owner {
                id: user_id.to_string(),
                display_name: user_id.to_string(),
            },
        })
    }

    async fn get_object(&self, bucket_name: &str, key: &str) -> Result<Vec<u8>, S3Error> {
        let buckets = self.buckets.read().unwrap();
        if let Some(bucket) = buckets.get(bucket_name) {
            if let Some(object) = bucket.get(key) {
                return Ok(object.clone());
            }
            return Err(S3Error::NoSuchKey(key.to_string()));
        }
        Err(S3Error::NoSuchBucket(bucket_name.to_string()))
    }

    async fn list_objects(&self, bucket_name: &str) -> Result<Vec<String>, S3Error> {
        let buckets = self.buckets.read().unwrap();
        if let Some(bucket) = buckets.get(bucket_name) {
            return Ok(bucket.keys().cloned().collect());
        }
        Err(S3Error::NoSuchBucket(bucket_name.to_string()))
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
