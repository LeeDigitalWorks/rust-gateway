use axum::{extract::Request, http::HeaderMap};

use super::v4_parser::parse_sign_v4;

pub const SIGN_V4_ALGORITHM: &str = "AWS4-HMAC-SHA256";

fn get_signed_headers(headers: &HeaderMap) -> String {
    let mut signed_headers = headers
        .keys()
        .map(|k| k.as_str().to_lowercase())
        .collect::<Vec<_>>();
    signed_headers.sort();
    signed_headers.join(";")
}

fn get_canonical_headers(headers: &HeaderMap) -> String {
    let mut canonical_headers = headers
        .iter()
        .map(|(k, v)| {
            format!(
                "{}:{}",
                k.as_str().to_lowercase(),
                v.to_str().unwrap().trim()
            )
        })
        .collect::<Vec<_>>();
    canonical_headers.sort();
    canonical_headers.join("\n")
}

fn get_scope(date: chrono::NaiveDateTime, region: &str) -> String {
    format!("{}/{}/s3/aws4_request", date.format("%Y%m%d"), region)
}

pub fn does_signature_match_v4(req: &Request) -> Result<(), s3_core::S3Error> {
    let auth_string = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();

    parse_sign_v4(auth_string.to_string(), req.headers())?;

    Ok(())
}
