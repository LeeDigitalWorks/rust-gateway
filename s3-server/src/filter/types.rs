use axum::{
    async_trait,
    body::{Body, Bytes},
};
use s3_core::{response::ResponseData, S3Error, S3Request};

#[async_trait]
pub trait Filter: Send + Sync {
    async fn handle(&mut self, data: &mut S3Data) -> Result<(), S3Error>;
}

#[derive(Debug)]
pub struct S3Data {
    pub req: axum::http::Request<axum::body::Bytes>,
    pub res: ResponseData,

    // Request ID
    pub request_id: String,

    pub access_key: String,
    pub secret_key: String,

    // Bucket the request is for
    pub bucket: Option<String>,

    // Bucket Name the request is for
    pub bucket_name: String,

    // Key the request is for
    pub key: String,

    // Host the request is for (with the bucket removed)
    pub host: String,

    pub operation: S3Request,
}

impl S3Data {
    pub fn new() -> Self {
        Self {
            req: axum::http::Request::new(Bytes::new()),
            res: ResponseData::new(),
            request_id: "".to_string(),
            access_key: "".to_string(),
            secret_key: "".to_string(),
            bucket: None,
            bucket_name: "".to_string(),
            key: "".to_string(),
            host: "".to_string(),
            operation: S3Request::Unknown,
        }
    }
}

pub async fn run_filters(
    filters: &mut Vec<Box<dyn Filter>>,
    data: &mut S3Data,
) -> Result<(), S3Error> {
    for filter in filters {
        filter.handle(data).await?;
    }

    tracing::debug!(operation = ?data.operation, "Request completed");
    Ok(())
}
