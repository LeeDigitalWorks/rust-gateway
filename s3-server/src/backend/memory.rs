use axum::async_trait;
use s3_core::response::ListBucketsResponse;
use s3_core::types::{BucketContainer, Owner};
use s3_core::{Bucket, S3Error};

use std::collections::HashMap;
use std::sync::RwLock;

pub struct InMemoryBackend {
    // The key is the bucket name, and the value is a map of object keys to object data.
    buckets: RwLock<HashMap<String, HashMap<String, Vec<u8>>>>,
    // The key is the owner ID and the value is a list of bucket names.
    owner_buckets: RwLock<HashMap<u64, Vec<Bucket>>>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self {
            buckets: RwLock::new(HashMap::new()),
            owner_buckets: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl crate::backend::Indexer for InMemoryBackend {
    async fn put_object(&self, bucket_name: &str, key: &str, data: Vec<u8>) -> Result<(), S3Error> {
        let mut buckets = self.buckets.write().unwrap();
        if let Some(bucket) = buckets.get_mut(bucket_name) {
            bucket.insert(key.to_string(), data);
            return Ok(());
        }
        Err(S3Error::NoSuchBucket(bucket_name.to_string()))
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

    async fn list_buckets(&self, user_id: &u64) -> Result<ListBucketsResponse, S3Error> {
        let buckets = self.owner_buckets.read().unwrap();
        let buckets = buckets.get(user_id).unwrap_or(&vec![]).to_owned();
        tracing::debug!(buckets = ?buckets, "Listed buckets");
        Ok(ListBucketsResponse {
            buckets: BucketContainer { buckets },
            owner: Owner {
                id: user_id.to_string(),
                display_name: user_id.to_string(),
            },
        })
    }

    async fn create_bucket(&self, bucket_name: &str, user_id: &u64) -> Result<(), S3Error> {
        let mut buckets = self.buckets.write().unwrap();
        if buckets.contains_key(bucket_name) {
            return Err(S3Error::BucketAlreadyExists(bucket_name.to_string()));
        }
        buckets.insert(bucket_name.to_string(), HashMap::new());
        let mut owner = self.owner_buckets.write().unwrap();
        if owner.contains_key(&user_id) {
            owner.get_mut(&user_id).unwrap().push(Bucket {
                name: bucket_name.to_string(),
                creation_date: chrono::Utc::now().to_string(),
            });
        } else {
            owner.insert(
                user_id.to_owned(),
                vec![Bucket {
                    name: bucket_name.to_string(),
                    creation_date: chrono::Utc::now().to_string(),
                }],
            );
        }
        Ok(())
    }

    async fn delete_bucket(&self, bucket_name: &str, user_id: &u64) -> Result<(), S3Error> {
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
}
