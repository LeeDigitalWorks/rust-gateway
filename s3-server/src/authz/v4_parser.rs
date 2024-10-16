use axum::http::HeaderMap;

pub fn parse_sign_v4(auth_string: String, headers: &HeaderMap) -> Result<(), s3_core::S3Error> {
    Ok(())
}
