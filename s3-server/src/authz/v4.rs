use axum::{extract::Request, http::HeaderMap};
use sha2::Digest;

use super::{v4_parser::parse_sign_v4, v4_utils::sum_hmac};

pub const SIGN_V4_ALGORITHM: &str = "AWS4-HMAC-SHA256";

fn get_scope(date: chrono::NaiveDateTime, region: &str) -> String {
    format!("{}/{}/s3/aws4_request", date.format("%Y%m%d"), region)
}

fn get_string_to_sign(
    date: chrono::NaiveDateTime,
    region: &str,
    canonical_request: &str,
) -> String {
    format!(
        "{}\n{}\n{}\n{}",
        SIGN_V4_ALGORITHM,
        date.format("%Y%m%dT%H%M%SZ"),
        get_scope(date, region),
        const_hex::encode(sha2::Sha256::digest(canonical_request.as_bytes()))
    )
}

fn get_signing_key(secret_key: String, date: chrono::NaiveDateTime, region: &str) -> Vec<u8> {
    let date_key = sum_hmac(
        format!("AWS4{}", secret_key).as_bytes().to_vec(),
        date.format("%Y%m%d").to_string().as_bytes().to_vec(),
    );
    let region_bytes = sum_hmac(date_key, region.as_bytes().to_vec());
    let service_key = sum_hmac(region_bytes, "s3".as_bytes().to_vec());
    sum_hmac(service_key, "aws4_request".as_bytes().to_vec())
}

fn get_signature(signing_key: Vec<u8>, string_to_sign: &str) -> String {
    const_hex::encode(sum_hmac(signing_key, string_to_sign.as_bytes().to_vec()))
}

pub fn does_signature_match_v4(req: &Request) -> Result<(), s3_core::S3Error> {
    let auth_string = req
        .headers()
        .get("Authorization")
        .map(|h| h.to_str().unwrap())
        .ok_or(s3_core::S3Error::InvalidArgument)?;

    parse_sign_v4(auth_string.to_string(), req.headers())?;

    let date_string = req
        .headers()
        .get("X-Amz-Date")
        .ok_or(s3_core::S3Error::InvalidRequest)?
        .to_str()
        .map_err(|_| s3_core::S3Error::InvalidRequest)?;

    let date = chrono::NaiveDateTime::parse_from_str(date_string, "%Y%m%dT%H%M%SZ")
        .map_err(|_| s3_core::S3Error::InvalidRequest)?;
    if chrono::Utc::now().naive_utc() - date > chrono::Duration::minutes(15) {
        return Err(s3_core::S3Error::RequestTimeTooSkewed);
    }

    // extract canonical request

    // extract signed headers

    // extract credential scope

    // extract string to sign

    // extract signing key

    // extract signature

    // compare signature

    Ok(())
}
