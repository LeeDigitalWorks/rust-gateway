use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use s3_core::S3Error;

use crate::{request::S3Info, server::AppState};

use super::v4::SIGN_V4_ALGORITHM;

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

fn is_signature(req: &Request) -> (bool, AuthType) {
    let auth_header = req.headers().get("Authorization");
    if let Some(auth_header) = auth_header {
        let auth_header = auth_header.to_str().unwrap();
        if auth_header.starts_with(SIGN_V4_ALGORITHM) {
            return (true, AuthType::SignedV4);
        }
    }

    (false, AuthType::Unknown)
}

fn get_auth_type(req: &Request) -> AuthType {
    if let (true, auth_type) = is_signature(req) {
        return auth_type;
    }

    AuthType::Unknown
}

pub async fn is_req_authenticated(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Response {
    let s3info = S3Info::from_request(&req, &state.keys);
    if let Err(e) = s3info {
        return axum::response::IntoResponse::into_response(e);
    }
    let s3info = s3info.unwrap();

    match get_auth_type(&req) {
        AuthType::SignedV4 => {
            match super::v4::does_signature_match_v4(&req, &s3info.secret_key) {
                Ok(_) => {}
                Err(e) => return axum::response::IntoResponse::into_response(e),
            }
        }
        _ => return axum::response::IntoResponse::into_response(S3Error::InternalError),
    }

    next.run(req).await
}
