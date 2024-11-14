use std::sync::Arc;

use axum::{async_trait, body::Bytes};
use s3_core::{response::ResponseData, S3Action, S3Error};

use crate::{backend::types, signature::Key};
#[async_trait]
pub trait Filter: Send + Sync {
    async fn handle(&self, data: &mut S3Data) -> Result<(), S3Error>;
}

pub struct S3Data {
    pub req: axum::http::Request<Bytes>,
    pub res: ResponseData,

    // Request ID
    pub request_id: String,

    pub auth_key: Key,

    // Bucket the request is for - backend bucket type
    pub bucket: Option<types::Bucket>,

    // Bucket Name the request is for
    pub bucket_name: String,

    // Key the request is for
    pub key: String,

    // Host the request is for (with the bucket removed)
    pub host: String,

    pub action: S3Action,
}

impl S3Data {
    pub fn new() -> Self {
        Self {
            req: axum::http::Request::new(Bytes::default()),
            res: ResponseData::new(),
            request_id: "".to_string(),
            auth_key: Key {
                access_key: "".to_string(),
                secret_key: "".to_string(),
                user_id: 0,
            },
            bucket: None,
            bucket_name: "".to_string(),
            key: "".to_string(),
            host: "".to_string(),
            action: S3Action::Unknown,
        }
    }
}

pub struct FilterChain {
    filters: Arc<Vec<Box<dyn Filter>>>,
}

impl FilterChain {
    pub fn new(filters: Vec<Box<dyn Filter>>) -> Self {
        Self {
            filters: Arc::new(filters),
        }
    }

    pub async fn run_filters(&self, data: &mut S3Data) -> Result<(), S3Error> {
        for filter in self.filters.iter() {
            filter.handle(data).await?;
        }

        tracing::debug!(operation = ?data.action, "Request completed");
        Ok(())
    }
}
