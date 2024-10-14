use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use s3_core::S3Error;

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

pub async fn is_req_authenticated(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    match get_auth_type(&req) {
        AuthType::SignedV4 => match super::v4::does_signature_match_v4(&req) {
            Ok(_) => {}
            Err(e) => return Ok(axum::response::IntoResponse::into_response(e)),
        },
        _ => {
            return Ok(axum::response::IntoResponse::into_response(
                S3Error::InternalError("Unknown auth type".to_string()),
            ))
        }
    }

    Ok(next.run(req).await)
}
