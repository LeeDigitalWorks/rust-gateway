use axum::async_trait;
use chrono::Timelike;
use s3_core::S3Error;
use uuid::{Timestamp, Uuid};

use std::collections::HashMap;

use super::super::types::Bucket;

#[async_trait]
impl crate::backend::IndexWriter for crate::backend::memory::InMemoryBackend {
    async fn create_bucket(&self, bucket_name: &str, user_id: &i64) -> Result<(), S3Error> {
        let mut buckets = self.buckets.write().unwrap();
        if buckets.contains_key(bucket_name) {
            return Err(S3Error::BucketAlreadyExists(bucket_name.to_string()));
        }
        buckets.insert(bucket_name.to_string(), HashMap::new());
        let mut owner = self.owner_buckets.write().unwrap();
        if owner.contains_key(&user_id) {
            owner.get_mut(&user_id).unwrap().push(Bucket {
                id: Uuid::new_v7(Timestamp::from_unix(
                    uuid::NoContext,
                    chrono::Utc::now().second().into(),
                    0,
                )),
                name: bucket_name.to_string(),
                created_at: chrono::Utc::now(),
                user_id: *user_id,
            });
        } else {
            owner.insert(
                user_id.to_owned(),
                vec![Bucket {
                    id: Uuid::new_v7(Timestamp::from_unix(
                        uuid::NoContext,
                        chrono::Utc::now().second().into(),
                        0,
                    )),
                    name: bucket_name.to_string(),
                    created_at: chrono::Utc::now(),
                    user_id: *user_id,
                }],
            );
        }
        Ok(())
    }

    async fn delete_bucket(&self, bucket_name: &str, user_id: &i64) -> Result<(), S3Error> {
        let mut owner = self.owner_buckets.write().unwrap();
        if let Some(buckets) = owner.get_mut(&user_id) {
            if let Some(index) = buckets.iter().position(|bucket| bucket.name == bucket_name) {
                buckets.remove(index);
            }
        }
        let mut buckets = self.buckets.write().unwrap();
        if buckets.remove(bucket_name).is_none() {
            return Err(S3Error::NoSuchBucket(bucket_name.to_string()));
        }
        Ok(())
    }

    async fn put_object(&self, bucket_name: &str, key: &str, data: Vec<u8>) -> Result<(), S3Error> {
        let mut buckets = self.buckets.write().unwrap();
        if let Some(bucket) = buckets.get_mut(bucket_name) {
            bucket.insert(key.to_string(), data);
            return Ok(());
        }
        Err(S3Error::NoSuchBucket(bucket_name.to_string()))
    }

    async fn delete_object(&self, bucket_name: &str, key: &str) -> Result<(), S3Error> {
        let mut buckets = self.buckets.write().unwrap();
        if let Some(bucket) = buckets.get_mut(bucket_name) {
            if bucket.remove(key).is_none() {
                return Err(S3Error::NoSuchKey(key.to_string()));
            }
            return Ok(());
        }
        Err(S3Error::NoSuchBucket(bucket_name.to_string()))
    }

    async fn delete_objects(&self, bucket_name: &str, keys: Vec<String>) -> Result<(), S3Error> {
        let mut buckets = self.buckets.write().unwrap();
        if let Some(bucket) = buckets.get_mut(bucket_name) {
            for key in keys {
                if bucket.remove(&key).is_none() {
                    return Err(S3Error::NoSuchKey(key));
                }
            }
            return Ok(());
        }
        Err(S3Error::NoSuchBucket(bucket_name.to_string()))
    }
}
