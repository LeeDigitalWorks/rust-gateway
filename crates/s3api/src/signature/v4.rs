use std::collections::HashMap;

use crate::{datatype, Credential};
use actix_web::http::header::HeaderMap;
use sha2::Digest;

use super::{v4_parser::parse_credential, v4_utils::sum_hmac};

pub const SIGN_V4_ALGORITHM: &str = "AWS4-HMAC-SHA256";

// get signed headers generates an alphabetically sorted list of headers semi-colon separated
pub fn get_signed_headers(headers: &HeaderMap) -> String {
    let mut signed_headers = headers
        .keys()
        .map(|k| k.as_str().to_lowercase())
        .collect::<Vec<_>>();
    signed_headers.sort();
    signed_headers.join(";")
}

// get canonical headers generates a canonical list of headers
pub fn get_canonical_headers(headers: &HeaderMap) -> String {
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

// get canonical request generates a canonical request
pub fn get_canonical_request(
    method: &str,
    uri: &str,
    query: &str,
    headers: &HeaderMap,
    payload_hash: &str,
) -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        method,
        uri,
        query,
        get_canonical_headers(headers),
        get_signed_headers(headers),
        payload_hash
    )
}

// get scope generates a string of a specific date, region, service
pub fn get_scope(date: chrono::NaiveDateTime, region: &str, service: &str) -> String {
    format!(
        "{}/{}/{}/aws4_request",
        date.format(datatype::YYYY_MM_DD),
        region,
        service
    )
}

// get string to sign generates a string to sign
pub fn get_string_to_sign(
    date: chrono::NaiveDateTime,
    region: &str,
    service: &str,
    canonical_request: &str,
) -> String {
    format!(
        "{}\n{}\n{}\n{}",
        SIGN_V4_ALGORITHM,
        date.format(datatype::ISO_8601_FORMAT),
        get_scope(date, region, service),
        const_hex::encode(sha2::Sha256::digest(canonical_request.as_bytes()))
    )
}

// get signing key generates a signing key
pub fn get_signing_key(secret_key: &str, date: chrono::NaiveDateTime, region: &str) -> Vec<u8> {
    let date_key = sum_hmac(
        format!("AWS4{}", secret_key).as_bytes().to_vec(),
        date.format(datatype::YYYY_MM_DD)
            .to_string()
            .as_bytes()
            .to_vec(),
    );
    let region_bytes = sum_hmac(date_key, region.as_bytes().to_vec());
    let service_key = sum_hmac(region_bytes, "s3".as_bytes().to_vec());
    sum_hmac(service_key, "aws4_request".as_bytes().to_vec())
}

// get signature generates a signature
pub fn get_signature(signing_key: Vec<u8>, string_to_sign: &str) -> String {
    const_hex::encode(sum_hmac(signing_key, string_to_sign.as_bytes().to_vec()))
}

// does policy signature match generates a boolean if the policy signature matches
pub fn does_policy_signature_match_v4(
    formValues: &HashMap<String, String>,
) -> Result<Credential, s3err::ApiErrorCode> {
    let credential = parse_credential(
        formValues
            .get("X-Amz-Credential")
            .unwrap_or(&"".to_string()),
    )?;
    let region = credential.scope.region;
    let date = chrono::NaiveDateTime::parse_from_str(
        formValues.get("X-Amz-Date").unwrap_or(&"".to_string()),
        datatype::ISO_8601_FORMAT,
    )
    .map_err(|_| s3err::ApiErrorCode::ErrMalformedDate)?;

    let signing_key = get_signing_key("", date, &region);
    let new_signature = get_signature(
        signing_key,
        &formValues.get("Policy").unwrap_or(&"".to_string()),
    );
    if new_signature != *formValues.get("X-Amz-Signature").unwrap_or(&"".to_string()) {
        return Err(s3err::ApiErrorCode::ErrSignatureDoesNotMatch);
    }

    Ok(Credential {
        ..Default::default()
    })
}
