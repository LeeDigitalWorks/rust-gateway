use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum S3Error {
    #[error("Access denied: {0}")]
    AccessDenied(String),
    #[error("Bucket already exists: {0}")]
    BucketAlreadyExists(String),
    #[error("Bucket not empty: {0}")]
    BucketNotEmpty(String),
    #[error("Invalid bucket name: {0}. Bucket names must be between 3 and 63 characters long and may contain only lowercase letters, numbers, hyphens, and periods.")]
    InvalidBucketName(String),
    #[error("InvalidAccessKeyId: {0}")]
    InvalidAccessKeyId(String),
    #[error("NoSuchBucket: {0}")]
    NoSuchBucket(String),
    #[error("NoSuchKey: {0}")]
    NoSuchKey(String),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
}

// Helper function to convert errors to HTTP status codes
pub fn error_to_http_status(error: &S3Error) -> u16 {
    match error {
        S3Error::AccessDenied(_) => 403,
        S3Error::BucketAlreadyExists(_) => 409,
        S3Error::BucketNotEmpty(_) => 409,
        S3Error::InternalError(_) => 500,
        S3Error::InvalidAccessKeyId(_) => 403,
        S3Error::InvalidRequest(_) => 400,
        S3Error::NoSuchBucket(_) => 404,
        S3Error::NoSuchKey(_) => 404,
        _ => 500,
    }
}

impl IntoResponse for S3Error {
    fn into_response(self) -> axum::response::Response<axum::body::Body> {
        let status = error_to_http_status(&self);
        let body = format!("{}", self);
        axum::http::Response::builder()
            .status(status)
            .body(axum::body::Body::from(body))
            .unwrap()
    }
}
