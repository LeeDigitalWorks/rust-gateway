use async_trait::async_trait;
use s3_core::response::ListBucketsResponse;
use s3_core::types::{BucketContainer, Owner};
use s3_core::{Bucket, S3Error};

use std::collections::HashMap;
use std::sync::RwLock;

pub struct InMemoryBackend {
    // The key is the bucket name, and the value is a map of object keys to object data.
    buckets: RwLock<HashMap<String, HashMap<String, Vec<u8>>>>,
    // The key is the owner ID and the value is a list of bucket names.
    owner_buckets: RwLock<HashMap<String, Vec<Bucket>>>,
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
impl crate::Backend for InMemoryBackend {
    async fn list_buckets(&self) -> Result<ListBucketsResponse, S3Error> {
        let buckets = self.owner_buckets.read().unwrap();
        let buckets = buckets
            .values()
            .flatten()
            .map(|bucket| Bucket {
                name: bucket.name.clone(),
                creation_date: bucket.creation_date.clone(),
            })
            .collect();
        Ok(ListBucketsResponse {
            buckets: BucketContainer { buckets },
            owner: Owner {
                id: "".to_string(),
                display_name: "".to_string(),
            },
        })
    }

    async fn create_bucket(&self, bucket_name: &str) -> Result<(), S3Error> {
        let mut owner = self.owner_buckets.write().unwrap();
        if owner.contains_key("") {
            owner.get_mut("").unwrap().push(Bucket {
                name: bucket_name.to_string(),
                creation_date: "".to_string(),
            });
        } else {
            owner.insert(
                "".to_string(),
                vec![Bucket {
                    name: bucket_name.to_string(),
                    creation_date: "".to_string(),
                }],
            );
        }
        let mut buckets = self.buckets.write().unwrap();
        if buckets.contains_key(bucket_name) {
            return Err(S3Error::BucketAlreadyExists(bucket_name.to_string()));
        }
        buckets.insert(bucket_name.to_string(), HashMap::new());
        Ok(())
    }

    async fn delete_bucket(&self, bucket_name: &str) -> Result<(), S3Error> {
        let mut owner = self.owner_buckets.write().unwrap();
        if let Some(buckets) = owner.get_mut("") {
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
