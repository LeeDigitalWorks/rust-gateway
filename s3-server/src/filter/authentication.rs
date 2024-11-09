use axum::async_trait;
use s3_core::S3Error;

use crate::authz::Authz;

use super::{Filter, S3Data};

pub struct AuthenticationFilter {
    authz: Authz,
}

impl AuthenticationFilter {
    pub fn new(authz: Authz) -> Self {
        Self { authz }
    }
}

#[async_trait]
impl Filter for AuthenticationFilter {
    async fn handle(&self, data: &mut S3Data) -> Result<(), S3Error> {
        data.auth_key = self.authz.check(&data.req).await?;
        Ok(())
    }
}
