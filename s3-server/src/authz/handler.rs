use axum::extract::Request;
use s3_core::S3Error;
use s3_iam::iam::GetKeyRequest;

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

fn is_signature(req: &Request<axum::body::Bytes>) -> (bool, AuthType) {
    let auth_header = req.headers().get("Authorization");
    if let Some(auth_header) = auth_header {
        let auth_header = auth_header.to_str().unwrap();
        if auth_header.starts_with(SIGN_V4_ALGORITHM) {
            return (true, AuthType::SignedV4);
        }
    }

    (false, AuthType::Unknown)
}

fn get_auth_type(req: &Request<axum::body::Bytes>) -> AuthType {
    if let (true, auth_type) = is_signature(req) {
        return auth_type;
    }

    AuthType::Unknown
}

pub struct Authz {
    pub client: s3_iam::iam::iam_client::IamClient<tonic::transport::Channel>,
}

impl Authz {
    pub fn new(client: s3_iam::iam::iam_client::IamClient<tonic::transport::Channel>) -> Self {
        Self { client }
    }

    pub async fn check(&mut self, req: &Request<axum::body::Bytes>) -> Result<(), S3Error> {
        match get_auth_type(req) {
            AuthType::SignedV4 => match self.does_signature_header_match_v4(req).await {
                Ok(_) => Ok(()),
                Err(_) => Err(S3Error::SignatureDoesNotMatch),
            },
            _ => Err(S3Error::NotImplemented),
        }
    }

    pub async fn get_key(&mut self, access_key: &str) -> Result<Key, S3Error> {
        let key = self
            .client
            .get_key(GetKeyRequest {
                access_key: access_key.to_string(),
            })
            .await;
        if let Err(_) = key {
            return Err(S3Error::InternalError);
        }

        let key = key.unwrap().into_inner().key;
        if let Some(key) = key {
            Ok(Key {
                access_key: key.access_key,
                secret_key: key.secret_key,
                user_id: key.user_id,
            })
        } else {
            Err(S3Error::InvalidAccessKeyId)
        }
    }

    pub async fn does_signature_header_match_v4(
        &mut self,
        req: &Request<axum::body::Bytes>,
    ) -> Result<(), s3_core::S3Error> {
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

        Ok(())
    }
}
