use axum::response::IntoResponse;
use bytes::Bytes;

#[derive(Debug, Serialize)]
pub enum S3Error {
    AccessDenied,
    BucketAlreadyExists(String),
    BucketNotEmpty,
    InvalidArgument,
    InvalidBucketName,
    InvalidAccessKeyId,
    MissingDateHeader,
    NoSuchBucket(String),
    NoSuchKey,
    InvalidRequest,
    InternalError,
    RequestTimeTooSkewed,
}

#[derive(Serialize, Debug)]
struct Error {
    #[serde(skip)]
    status: u16,
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Resource")]
    resource: String,
    #[serde(rename = "RequestId")]
    request_id: String,
}

fn s3error_to_error(error: &S3Error) -> Error {
    match error {
        S3Error::AccessDenied => Error {
            status: 403,
            code: "AccessDenied".to_string(),
            message: "Access Denied".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        S3Error::BucketAlreadyExists(bucket) => Error {
            status: 409,
            code: "BucketAlreadyExists".to_string(),
            message: format!("The requested bucket name is not available. The bucket namespace is shared by all users of the system. Please select a different name and try again. Bucket name: '{}'", bucket),
            resource: bucket.to_string(),
            request_id: "".to_string(),
        },
        S3Error::InvalidRequest => Error {
            status: 400,
            code: "InvalidRequest".to_string(),
            message: "Invalid Request".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        _ => Error {
            status: 500,
            code: "InternalError".to_string(),
            message: "Internal Server Error".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
    }
}

impl IntoResponse for S3Error {
    fn into_response(self) -> axum::response::Response<axum::body::Body> {
        let error = s3error_to_error(&self);
        let bytes: Bytes = quick_xml::se::to_string(&error).unwrap().into();
        axum::http::Response::builder()
            .status(error.status)
            .body(axum::body::Body::from(bytes))
            .unwrap()
    }
}
