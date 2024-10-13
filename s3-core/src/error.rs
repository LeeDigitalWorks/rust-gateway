use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum S3Error {
    #[error("Bucket already exists: {0}")]
    BucketAlreadyExists(String),
    #[error("NoSuchBucket: {0}")]
    NoSuchBucket(String),
    #[error("NoSuchKey: {0}")]
    NoSuchKey(String),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    #[error("Access denied: {0}")]
    AccessDenied(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
}

// Helper function to convert errors to HTTP status codes
pub fn error_to_http_status(error: &S3Error) -> u16 {
    match error {
        S3Error::NoSuchBucket(_) => 404,
        S3Error::InvalidRequest(_) => 400,
        S3Error::AuthenticationError(_) => 401,
        S3Error::AccessDenied(_) => 403,
        S3Error::InternalError(_) => 500,
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
