use std::{collections::HashMap, sync::Arc};

use axum::async_trait;
use s3_core::S3Error;
use tokio::sync::RwLock;

use crate::authz::Key;

use super::{Filter, S3Data};

pub struct SecretKeyFilter {
    keys: Arc<RwLock<HashMap<String, Key>>>,
}

impl SecretKeyFilter {
    pub fn new(keys: Arc<RwLock<HashMap<String, Key>>>) -> Self {
        Self { keys }
    }
}

#[async_trait]
impl Filter for SecretKeyFilter {
    async fn handle(&self, data: &mut S3Data) -> Result<(), S3Error> {
        if data.auth_key.secret_key.is_empty()
            || self
                .keys
                .read()
                .await
                .get(&data.auth_key.access_key)
                .is_none()
        {
            return Err(S3Error::InvalidAccessKeyId);
        }
        Ok(())
    }
}
