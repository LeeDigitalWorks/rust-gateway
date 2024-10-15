use super::types::ObjectMetadata;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum S3Request {
    #[default]
    Unknown,
    ListBuckets,
    CreateBucket {
        name: String,
    },
    DeleteBucket {
        name: String,
    },
    ListObjects {
        bucket: String,
        prefix: Option<String>,
        delimiter: Option<String>,
        max_keys: Option<usize>,
        continuation_token: Option<String>,
    },
    GetObject {
        bucket: String,
        key: String,
    },
    PutObject {
        bucket: String,
        key: String,
        data: Vec<u8>,
        metadata: ObjectMetadata,
    },
    DeleteObject {
        bucket: String,
        key: String,
    },
    HeadObject {
        bucket: String,
        key: String,
    },
}

impl S3Request {
    pub fn bucket_name(&self) -> Option<&str> {
        match self {
            S3Request::CreateBucket { name } => Some(name),
            S3Request::DeleteBucket { name } => Some(name),
            S3Request::ListObjects { bucket, .. } => Some(bucket),
            S3Request::GetObject { bucket, .. } => Some(bucket),
            S3Request::PutObject { bucket, .. } => Some(bucket),
            S3Request::DeleteObject { bucket, .. } => Some(bucket),
            S3Request::HeadObject { bucket, .. } => Some(bucket),
            _ => None,
        }
    }

    pub fn object_key(&self) -> Option<&str> {
        match self {
            S3Request::GetObject { key, .. } => Some(key),
            S3Request::PutObject { key, .. } => Some(key),
            S3Request::DeleteObject { key, .. } => Some(key),
            S3Request::HeadObject { key, .. } => Some(key),
            _ => None,
        }
    }
}
