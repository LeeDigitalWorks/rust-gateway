use axum::response::IntoResponse;
use bytes::Bytes;

#[derive(Debug, Serialize)]
pub enum S3Error {
    AccessDenied,
    AuthorizationHeaderMalformed,
    BucketAlreadyExists(String),
    BucketNotEmpty,
    InvalidArgument,
    InvalidBucketName,
    InvalidAccessKeyId,
    MissingDateHeader,
    NoSuchBucket(String),
    NoSuchKey(String),
    InvalidRequest,
    InternalError,
    NotImplemented,
    RequestTimeTooSkewed,
    SignatureDoesNotMatch,
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
            status: http::StatusCode::FORBIDDEN.into(),
            code: "AccessDenied".to_string(),
            message: "Access Denied".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        S3Error::BucketAlreadyExists(bucket) => Error {
            status: http::StatusCode::CONFLICT.into(),
            code: "BucketAlreadyExists".to_string(),
            message: format!("The requested bucket name is not available. The bucket namespace is shared by all users of the system. Please select a different name and try again. Bucket name: '{}'", bucket),
            resource: bucket.to_string(),
            request_id: "".to_string(),
        },
        S3Error::InvalidRequest => Error {
            status: http::StatusCode::BAD_REQUEST.into(),
            code: "InvalidRequest".to_string(),
            message: "Invalid Request".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        S3Error::SignatureDoesNotMatch => Error {
            status: http::StatusCode::FORBIDDEN.into(),
            code: "SignatureDoesNotMatch".to_string(),
            message: "The request signature we calculated does not match the signature you provided. Check your key and signing method.".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        S3Error::InvalidAccessKeyId => Error {
            status: http::StatusCode::FORBIDDEN.into(),
            code: "InvalidAccessKeyId".to_string(),
            message: "The AWS Access Key Id you provided does not exist in our records.".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        S3Error::NoSuchBucket(bucket) => Error {
            status: http::StatusCode::NOT_FOUND.into(),
            code: "NoSuchBucket".to_string(),
            message: format!("The specified bucket does not exist. Bucket: '{}'", bucket),
            resource: bucket.to_string(),
            request_id: "".to_string(),
        },
        S3Error::NoSuchKey(key) => Error {
            status: http::StatusCode::NOT_FOUND.into(),
            code: "NoSuchKey".to_string(),
            message: format!("The specified key does not exist. Key: '{}'", key),
            resource: key.to_string(),
            request_id: "".to_string(),
        },
        S3Error::BucketNotEmpty => Error {
            status: http::StatusCode::CONFLICT.into(),
            code: "BucketNotEmpty".to_string(),
            message: "The bucket you tried to delete is not empty.".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        S3Error::InvalidBucketName => Error {
            status: http::StatusCode::BAD_REQUEST.into(),
            code: "InvalidBucketName".to_string(),
            message: "The specified bucket is not valid.".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        S3Error::AuthorizationHeaderMalformed => Error {
            status: http::StatusCode::BAD_REQUEST.into(),
            code: "AuthorizationHeaderMalformed".to_string(),
            message: "The authorization header is malformed".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        S3Error::MissingDateHeader => Error {
            status: http::StatusCode::BAD_REQUEST.into(),
            code: "MissingDateHeader".to_string(),
            message: "Missing required header for this request: Date".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        S3Error::RequestTimeTooSkewed => Error {
            status: http::StatusCode::BAD_REQUEST.into(),
            code: "RequestTimeTooSkewed".to_string(),
            message: "The difference between the request time and the server's time is too large.".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        S3Error::NotImplemented => Error {
            status: http::StatusCode::NOT_IMPLEMENTED.into(),
            code: "NotImplemented".to_string(),
            message: "A header you provided implies functionality that is not implemented.".to_string(),
            resource: "".to_string(),
            request_id: "".to_string(),
        },
        _ => Error {
            status: http::StatusCode::INTERNAL_SERVER_ERROR.into(),
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
