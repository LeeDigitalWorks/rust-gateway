use std::{collections::HashMap, sync::Arc};

use axum::extract::Request;
use s3_core::S3Error;

use super::access_key::parse_access_key;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct S3Info {
    pub bucket: String,
    pub key: String,
    pub region: String,
    pub access_key: String,
    pub secret_key: String,
}

impl S3Info {
    pub fn from_request(
        req: &Request,
        keys: &Arc<HashMap<String, s3_iam::iampb::iam::Key>>,
    ) -> Result<Self, S3Error> {
        let access_key = parse_access_key(req)?;
        let secret_key = keys
            .get(&access_key)
            .map(|key| key.secret_key.clone())
            .unwrap_or_default();
        let bucket_name = req.uri().path().trim_start_matches('/').split('/').next();
        let key = req.uri().path().trim_start_matches('/').split('/').nth(1);

        tracing::debug!(
            access_key = access_key.as_str(),
            secret_key = secret_key.as_str(),
            bucket = bucket_name.unwrap_or_default(),
            key = key.unwrap_or_default(),
            "Parsed S3Info"
        );

        Ok(Self {
            access_key,
            secret_key,
            bucket: bucket_name.unwrap_or_default().to_string(),
            key: key.unwrap_or_default().to_string(),
            ..Default::default()
        })
    }
}
