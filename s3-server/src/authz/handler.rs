use std::{collections::HashMap, sync::Arc};

use axum::extract::Request;
use s3_core::S3Error;
use s3_iam::iam::GetKeyRequest;
use tokio::sync::RwLock;

use super::{
    v4::{
        get_canonical_request, get_signature, get_signing_key, get_string_to_sign,
        parse_auth_header, parse_date, SIGN_V4_ALGORITHM,
    },
    Key,
};

enum AuthType {
    Unknown,
    Anonymous,
    PresignedV4,
    PresignedV2,
    PostPolicy,
    StreamingSigned,
    SignedV4,
    Signedv2,
}

fn is_signature(req: &Request<reqwest::Body>) -> (bool, AuthType) {
    let auth_header = req.headers().get("Authorization");
    if let Some(auth_header) = auth_header {
        let auth_header = auth_header.to_str().unwrap();
        if auth_header.starts_with(SIGN_V4_ALGORITHM) {
            return (true, AuthType::SignedV4);
        }
    }

    (false, AuthType::Unknown)
}

fn get_auth_type(req: &Request<reqwest::Body>) -> AuthType {
    if let (true, auth_type) = is_signature(req) {
        return auth_type;
    }

    AuthType::Unknown
}

pub struct Authz {
    pub keys: Arc<RwLock<HashMap<String, Key>>>,
}

impl Authz {
    pub fn new(keys: Arc<RwLock<HashMap<String, Key>>>) -> Self {
        Self { keys }
    }

    pub async fn check(&mut self, req: &Request<reqwest::Body>) -> Result<Key, S3Error> {
        match get_auth_type(req) {
            AuthType::SignedV4 => match self.check_signature_header_match_v4(req).await {
                Ok(key) => Ok(key),
                Err(e) => Err(e),
            },
            _ => Err(S3Error::NotImplemented),
        }
    }

    pub async fn get_key(&mut self, access_key: &str) -> Result<Key, S3Error> {
        let cache = self.keys.read().await;
        if let Some(key) = cache.get(access_key) {
            return Ok(key.clone());
        }
        Err(S3Error::InvalidAccessKeyId)
    }

    pub async fn check_signature_header_match_v4(
        &mut self,
        req: &Request<reqwest::Body>,
    ) -> Result<Key, s3_core::S3Error> {
        let auth_string = req
            .headers()
            .get("Authorization")
            .map(|h| h.to_str().unwrap())
            .ok_or(s3_core::S3Error::AuthorizationHeaderMalformed)?;
        let auth_header = parse_auth_header(auth_string)?;

        let date = parse_date(req.headers())?;

        let key = self.get_key(&auth_header.credential.access_key).await?;

        let canonical_request = get_canonical_request(req, &auth_header.signed_headers);

        let string_to_sign =
            get_string_to_sign(date, &auth_header.credential.region, &canonical_request);

        let signing_key = get_signing_key(
            &key.secret_key,
            auth_header.credential.date,
            &auth_header.credential.region,
        );

        let signature = get_signature(signing_key, &string_to_sign);

        // compare signature
        let auth_signature = auth_string.split("Signature=").collect::<Vec<&str>>()[1].trim();
        if subtle::ConstantTimeEq::ct_eq(signature.as_bytes(), auth_signature.as_bytes())
            .unwrap_u8()
            .eq(&0)
        {
            tracing::debug!(
                canonical_request = canonical_request,
                string_to_sign = string_to_sign,
                signature = signature,
                auth_signature = auth_signature,
                "Signature does not match"
            );

            return Err(s3_core::S3Error::SignatureDoesNotMatch);
        }

        Ok(key)
    }
}
