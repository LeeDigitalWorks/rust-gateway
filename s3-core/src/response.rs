use std::collections::HashMap;

use axum::response::IntoResponse;
use bytes::Bytes;
use serde_derive::Deserialize;

use crate::types::{BucketContainer, Owner};

pub struct ResponseData {
    bytes: Bytes,
    status_code: u16,
    headers: HashMap<String, String>,
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

impl ListBucketsResponse {
    pub fn into_response(&self) -> ResponseData {
        ResponseData {
            bytes: quick_xml::se::to_string(self).unwrap().into(),
            status_code: 200,
            headers: HashMap::new(),
        }
    }
}
