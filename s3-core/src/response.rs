use std::collections::HashMap;

use axum::response::IntoResponse;
use bytes::Bytes;
use serde_derive::Deserialize;

use crate::types::{BucketContainer, Owner};

#[derive(Debug, Clone)]
pub struct ResponseData {
    pub bytes: Bytes,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
}

impl ResponseData {
    pub fn new() -> Self {
        Self {
            bytes: Bytes::new(),
            status_code: http::StatusCode::NOT_IMPLEMENTED.into(),
            headers: HashMap::new(),
        }
    }

    pub fn with_bytes(&mut self, bytes: Bytes) -> &mut Self {
        self.bytes = bytes;
        self
    }

    pub fn with_status_code(&mut self, status_code: u16) -> &mut Self {
        self.status_code = status_code;
        self
    }

    pub fn with_header(&mut self, key: String, value: String) -> &mut Self {
        self.headers.insert(key, value);
        self
    }
}

impl IntoResponse for ResponseData {
    fn into_response(self) -> axum::response::Response<axum::body::Body> {
        let mut builder = axum::http::Response::builder().status(self.status_code);

        for (key, value) in self.headers {
            builder = builder.header(key, value);
        }

        builder
            .body(axum::body::Body::from(self.bytes))
            .unwrap_or_default()
    }
}

#[derive(Clone, Default, Deserialize, Debug, Serialize)]
#[serde(rename_all = "PascalCase", rename = "ListAllMyBucketsResult")]
pub struct ListBucketsResponse {
    pub owner: Owner,
    pub buckets: BucketContainer,
}

impl IntoResponse for ListBucketsResponse {
    fn into_response(self) -> axum::response::Response<axum::body::Body> {
        ResponseData {
            bytes: quick_xml::se::to_string(&self).unwrap().into(),
            status_code: 200,
            headers: HashMap::new(),
        }
        .into_response()
    }
}
