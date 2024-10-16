use std::collections::BTreeMap;

use axum::{extract::Request, http::HeaderMap};
use sha2::Digest;

use super::{v4_parser::parse_sign_v4, v4_utils::sum_hmac};

pub static SIGN_V4_ALGORITHM: &str = "AWS4-HMAC-SHA256";
static UNSIGNED_HASH: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

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

fn get_signing_key(secret_key: &str, date: chrono::NaiveDateTime, region: &str) -> Vec<u8> {
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

fn get_signed_headers(req: &Request) -> Vec<(&str, String)> {
    req.headers()
        .iter()
        .filter_map(|(key, val)| match key.as_str() {
            "authorization" | "content-length" | "user-agent" => None,
            _ => val.to_str().ok().map(|v| (key.as_str(), v)),
        })
        .chain(std::iter::once((
            "host",
            req.uri().authority().map(|a| a.as_str()).unwrap_or(""),
        )))
        .fold(BTreeMap::<&str, String>::new(), |mut map, (key, val)| {
            map.entry(key)
                .and_modify(|v| {
                    *v = [v, val].join(",");
                })
                .or_insert_with(|| val.to_string());
            map
        })
        .into_iter()
        .collect()
}

fn get_canonical_request(req: &Request) -> String {
    let method = req.method().as_str();

    // TODO: URI encode
    let uri = req.uri().path_and_query().unwrap().as_str();

    // TODO: URI encode
    let query = req.uri().query().unwrap_or("");

    // TODO: Lowercase headers and trim values
    let headers = req
        .headers()
        .iter()
        .map(|(k, v)| format!("{}:{}", k, v.to_str().unwrap()))
        .collect::<Vec<String>>()
        .join("\n");
    let signed_headers = get_signed_headers(req)
        .iter()
        .map(|(k, _)| *k)
        .collect::<Vec<&str>>()
        .join(";");
    let payload_hash = req
        .headers()
        .get("X-Amz-Content-Sha256")
        .unwrap()
        .to_str()
        .unwrap();

    format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        method, uri, query, headers, signed_headers, payload_hash
    )
}

pub fn does_signature_match_v4(
    req: &Request,
    secret_key: &str,
    region: &str,
) -> Result<(), s3_core::S3Error> {
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
    let canonical_request = get_canonical_request(req);

    // extract string to sign
    let string_to_sign = get_string_to_sign(date, region, &canonical_request);

    // extract signing key
    let signing_key = get_signing_key(secret_key, date, region);

    // extract signature
    let signature = get_signature(signing_key, &string_to_sign);

    // compare signature
    let auth_signature = auth_string.split("Signature=").collect::<Vec<&str>>()[1].trim();
    if signature != auth_signature {
        return Err(s3_core::S3Error::InvalidRequest);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_canonical_request() {
        let req = Request::builder()
            .method("GET")
            .header("Host", "examplebucket.s3.amazonaws.com")
            .header("Range", "bytes=0-9")
            .header("x-amz-content-sha256", UNSIGNED_HASH)
            .header("x-amx-date", "20130524T000000Z")
            .uri("/test.txt")
            .body(axum::body::Body::empty())
            .unwrap();

        let canonical_request = get_canonical_request(&req);
        assert_eq!(
            canonical_request,
            "GET\n/test.txt\n\nhost:examplebucket.s3.amazonaws.com\nrange:bytes=0-9\nx-amz-content-sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\nx-amx-date:20130524T000000Z\n\nhost;range;x-amz-content-sha256;x-amx-date\ne3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
