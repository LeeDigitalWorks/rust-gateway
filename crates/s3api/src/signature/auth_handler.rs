use std::collections::HashMap;

use actix_web::{
    http::header::HeaderMap,
    web::{self, Query},
    HttpRequest,
};

use crate::Credential;

use super::{
    streaming_signature_v4::STREAMING_CONTENT_SHA_256, v2::SIGN_V2_ALGORITHM, v4::SIGN_V4_ALGORITHM,
};

pub enum AuthType {
    Unknown,
    Anonymous,
    PresignedV4,
    PresignedV2,
    PostPolicy,
    StreamingSigned,
    SignedV4,
    SignedV2,
}

fn is_request_signature(headers: &HeaderMap) -> (bool, AuthType) {
    let auth_header = headers.get("Authorization");
    if let Some(auth_header) = auth_header {
        let auth_header = auth_header.to_str().unwrap();
        if auth_header.starts_with(SIGN_V4_ALGORITHM) {
            return (true, AuthType::SignedV4);
        } else if auth_header.starts_with(SIGN_V2_ALGORITHM) {
            return (true, AuthType::SignedV2);
        }
    }
    (false, AuthType::Unknown)
}

fn is_request_presigned(query: Query<HashMap<String, String>>) -> (bool, AuthType) {
    if query.contains_key("X-Amz-Credential") {
        return (true, AuthType::PresignedV4);
    } else if query.contains_key("AWSAccessKeyId") {
        return (true, AuthType::PresignedV2);
    }
    (false, AuthType::Unknown)
}

fn is_request_post_policy_signature(req: &HttpRequest) -> bool {
    if req.method() != "POST" {
        return false;
    }
    let content_type = req.headers().get("Content-Type");
    if let Some(content_type) = content_type {
        if content_type
            .to_str()
            .unwrap()
            .contains("multipart/form-data")
        {
            return true;
        }
    }
    false
}

fn is_request_sign_streaming_v4(req: &HttpRequest) -> bool {
    req.headers()
        .get("X-Amz-Content-Sha256")
        .is_some_and(|v| v.to_str().unwrap().eq(STREAMING_CONTENT_SHA_256))
        && req.method() == "PUT"
}

pub fn get_request_auth_type(req: &HttpRequest) -> AuthType {
    if is_request_sign_streaming_v4(req) {
        return AuthType::StreamingSigned;
    }
    if let (true, auth_type) = is_request_signature(req.headers()) {
        return auth_type;
    }
    let query = Query::from_query(req.query_string())
        .map_err(|_| ())
        .unwrap();
    if let (true, auth_type) = is_request_presigned(query) {
        return auth_type;
    }
    if let None = req.headers().get("Authorization") {
        return AuthType::Anonymous;
    }
    AuthType::Unknown
}

pub fn is_req_authenticated(
    req: &HttpRequest,
    body: web::Bytes,
) -> Result<Credential, s3err::ApiErrorCode> {
    // read entire body
    let body = body.to_vec();

    if let Some(header) = req.headers().get("Content-Md5") {
        // check if Content-Md5 matches md5 sum of body
        
    }

    match get_request_auth_type(req) {
        AuthType::PresignedV4 => return Err(s3err::ApiErrorCode::ErrAccessDenied),
        AuthType::SignedV4 => return Err(s3err::ApiErrorCode::ErrAccessDenied),
        AuthType::PresignedV2 => return Err(s3err::ApiErrorCode::ErrAccessDenied),
        AuthType::SignedV2 => return Err(s3err::ApiErrorCode::ErrAccessDenied),
        AuthType::StreamingSigned => return Err(s3err::ApiErrorCode::ErrAccessDenied),
        _ => return Err(s3err::ApiErrorCode::ErrAccessDenied),
    }
}
