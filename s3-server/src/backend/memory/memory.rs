use axum::async_trait;
use chrono::Timelike;
use s3_core::response::ListBucketsResponse;
use s3_core::types::{BucketContainer, Owner};
use s3_core::S3Error;
use uuid::{Timestamp, Uuid};

use std::collections::HashMap;
use std::sync::RwLock;

use super::super::types::Bucket;

pub struct InMemoryBackend {
    // The key is the bucket name, and the value is a map of object keys to object data.
    pub buckets: RwLock<HashMap<String, HashMap<String, Vec<u8>>>>,
    // The key is the owner ID and the value is a list of bucket names.
    pub owner_buckets: RwLock<HashMap<u64, Vec<Bucket>>>,
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
impl crate::backend::Indexer for InMemoryBackend {}
