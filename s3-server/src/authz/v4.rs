use std::{collections::BTreeMap, usize};

use axum::{extract::Request, http::HeaderMap};
use sha2::Digest;

use crate::authz::v4_utils::sum_sha256;

use super::v4_utils::hmac_sha256;

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
        const_hex::encode(sum_sha256(canonical_request))
    )
}

fn get_signing_key(secret_key: &str, date: chrono::NaiveDateTime, region: &str) -> Vec<u8> {
    let date_key = hmac_sha256(
        ["AWS4", secret_key].concat(),
        date.format("%Y%m%d").to_string(),
    );
    let date_region_key = hmac_sha256(date_key, region);
    let date_region_service_key = hmac_sha256(date_region_key, "s3");
    hmac_sha256(date_region_service_key, "aws4_request").to_vec()
}

fn get_signature(signing_key: Vec<u8>, string_to_sign: &str) -> String {
    const_hex::encode(hmac_sha256(signing_key, string_to_sign))
}

fn get_signed_headers(req: &Request) -> Vec<(&str, String)> {
    req.headers()
        .iter()
        .filter_map(|(key, val)| match key.as_str() {
            "authorization" | "content-length" | "user-agent" | "accept-encoding" => None,
            _ => val.to_str().ok().map(|v| (key.as_str(), v)),
        })
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

fn get_payload_hash(req: &Request) -> String {
    if let Some(hash) = req.headers().get("x-amz-content-sha256") {
        return hash.to_str().unwrap().to_string();
    }
    UNSIGNED_HASH.to_string()
}

fn url_encode(s: &str, encode_slash: bool) -> String {
    fn is_unreserved(c: char, encode_slash: bool) -> bool {
        c.is_ascii_alphanumeric()
            || matches!(c, '-' | '_' | '.' | '~')
            || (c == '/' && !encode_slash)
    }

    s.chars()
        .map(|c| {
            if is_unreserved(c, encode_slash) {
                c.to_string()
            } else {
                format!("%{:02X}", c as u8)
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn get_canonical_request(req: &Request) -> String {
    let method = req.method().as_str();

    let uri = url_encode(req.uri().path(), false);

    let mut query = req.uri().query().unwrap_or("").to_string();
    if !query.is_empty() {
        let mut query_params = query
            .split('&')
            .map(|param| {
                let mut parts = param.split('=');
                let key = parts.next().unwrap();
                let val = parts.next().unwrap_or("");
                (url_encode(key, true), url_encode(val, true))
            })
            .collect::<Vec<_>>();
        query_params.sort_unstable();
        query = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");
    }

    let mut headers = get_signed_headers(req)
        .iter()
        .map(|(k, v)| format!("{}:{}", k.to_lowercase(), v.trim()))
        .collect::<Vec<String>>();
    headers.sort_unstable();
    let headers = headers.join("\n");

    let mut signed_headers = get_signed_headers(req)
        .iter()
        .map(|(k, _)| k.to_lowercase())
        .collect::<Vec<_>>();
    signed_headers.sort_unstable();
    let signed_headers = signed_headers.join(";");

    let payload_hash = get_payload_hash(req);

    format!(
        "{}\n{}\n{}\n{}\n\n{}\n{}",
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

    let canonical_request = get_canonical_request(req);

    let string_to_sign = get_string_to_sign(date, region, &canonical_request);

    let signing_key = get_signing_key(secret_key, date, region);

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
        return Err(s3_core::S3Error::InvalidRequest);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static ACCESS_KEY: &str = "AKIAIOSFODNN7EXAMPLE";
    static SECRET_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";

    #[test]
    fn test_get_scope() {
        let scope = get_scope(
            chrono::NaiveDateTime::parse_from_str("20130524T000000Z", "%Y%m%dT%H%M%SZ").unwrap(),
            "us-east-1",
        );
        assert_eq!(scope, "20130524/us-east-1/s3/aws4_request");
    }

    #[test]
    fn test_get_canonical_request() {
        let req = Request::builder()
            .method("GET")
            .header("Host", "examplebucket.s3.amazonaws.com")
            .header("Range", "bytes=0-9")
            .header("x-amz-content-sha256", UNSIGNED_HASH)
            .header("x-amz-date", "20130524T000000Z")
            .uri("/test.txt")
            .body(axum::body::Body::empty())
            .unwrap();

        let canonical_request = get_canonical_request(&req);
        assert_eq!(
            canonical_request,
            "GET\n/test.txt\n\nhost:examplebucket.s3.amazonaws.com\nrange:bytes=0-9\nx-amz-content-sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\nx-amz-date:20130524T000000Z\n\nhost;range;x-amz-content-sha256;x-amz-date\ne3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_get_object() {
        let signing_key = get_signing_key(
            SECRET_KEY,
            chrono::NaiveDateTime::parse_from_str("20130524T000000Z", "%Y%m%dT%H%M%SZ").unwrap(),
            "us-east-1",
        );

        let string_to_sign = "AWS4-HMAC-SHA256\n20130524T000000Z\n20130524/us-east-1/s3/aws4_request\n7344ae5b7ee6c3e7e6b0fe0640412a37625d1fbfff95c48bbb2dc43964946972";

        let signature = get_signature(signing_key, &string_to_sign);
        assert_eq!(
            signature,
            "f0e8bdb87c964420e857bd35b5d6ed310bd44f0170aba48dd91039c6036bdb41"
        );
    }

    #[test]
    fn test_put_object() {
        let req = Request::builder()
            .method("PUT")
            .header("Host", "examplebucket.s3.amazonaws.com")
            .header(
                "x-amz-content-sha256",
                "44ce7dd67c959e0d3524ffac1771dfbba87d2b6b4b4e99e42034a8b803f8b072",
            )
            .header("Date", "Fri, 24 May 2013 00:00:00 GMT")
            .header("x-amz-date", "20130524T000000Z")
            .header("x-amz-storage-class", "REDUCED_REDUNDANCY")
            .uri("/test$file.text")
            .body(axum::body::Body::from("Welcome to Amazon S3."))
            .unwrap();

        let canonical_request = get_canonical_request(&req);
        assert_eq!(
            canonical_request,
            "PUT\n/test%24file.text\n\ndate:Fri, 24 May 2013 00:00:00 GMT\nhost:examplebucket.s3.amazonaws.com\nx-amz-content-sha256:44ce7dd67c959e0d3524ffac1771dfbba87d2b6b4b4e99e42034a8b803f8b072\nx-amz-date:20130524T000000Z\nx-amz-storage-class:REDUCED_REDUNDANCY\n\ndate;host;x-amz-content-sha256;x-amz-date;x-amz-storage-class\n44ce7dd67c959e0d3524ffac1771dfbba87d2b6b4b4e99e42034a8b803f8b072"
        );

        let string_to_sign = get_string_to_sign(
            chrono::NaiveDateTime::parse_from_str("20130524T000000Z", "%Y%m%dT%H%M%SZ").unwrap(),
            "us-east-1",
            &canonical_request,
        );

        let signing_key = get_signing_key(
            SECRET_KEY,
            chrono::NaiveDateTime::parse_from_str("20130524T000000Z", "%Y%m%dT%H%M%SZ").unwrap(),
            "us-east-1",
        );

        let signature = get_signature(signing_key, &string_to_sign);

        assert_eq!(
            signature,
            "98ad721746da40c64f1a55b78f14c238d841ea1380cd77a1b5971af0ece108bd"
        );
    }

    #[test]
    fn test_get_bucket_lifecycle() {
        let req = Request::builder()
            .method("GET")
            .header("Host", "examplebucket.s3.amazonaws.com")
            .header("x-amz-content-sha256", UNSIGNED_HASH)
            .header("x-amz-date", "20130524T000000Z")
            .uri("/?lifecycle")
            .body(axum::body::Body::empty())
            .unwrap();

        let canonical_request = get_canonical_request(&req);
        assert_eq!(
            canonical_request,
            "GET\n/\nlifecycle=\nhost:examplebucket.s3.amazonaws.com\nx-amz-content-sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\nx-amz-date:20130524T000000Z\n\nhost;x-amz-content-sha256;x-amz-date\ne3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );

        let string_to_sign = get_string_to_sign(
            chrono::NaiveDateTime::parse_from_str("20130524T000000Z", "%Y%m%dT%H%M%SZ").unwrap(),
            "us-east-1",
            &canonical_request,
        );

        let signing_key = get_signing_key(
            SECRET_KEY,
            chrono::NaiveDateTime::parse_from_str("20130524T000000Z", "%Y%m%dT%H%M%SZ").unwrap(),
            "us-east-1",
        );

        let signature = get_signature(signing_key, &string_to_sign);

        assert_eq!(
            signature,
            "fea454ca298b7da1c68078a5d1bdbfbbe0d65c699e0f91ac7a200a0136783543"
        );
    }

    #[test]
    fn test_list_objects() {
        let req = Request::builder()
            .method("GET")
            .header("Host", "examplebucket.s3.amazonaws.com")
            .header("x-amz-content-sha256", UNSIGNED_HASH)
            .header("x-amz-date", "20130524T000000Z")
            .uri("/?max-keys=2&prefix=J")
            .body(axum::body::Body::empty())
            .unwrap();

        let canonical_request = get_canonical_request(&req);
        assert_eq!(
            canonical_request,
            "GET\n/\nmax-keys=2&prefix=J\nhost:examplebucket.s3.amazonaws.com\nx-amz-content-sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\nx-amz-date:20130524T000000Z\n\nhost;x-amz-content-sha256;x-amz-date\ne3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );

        let string_to_sign = get_string_to_sign(
            chrono::NaiveDateTime::parse_from_str("20130524T000000Z", "%Y%m%dT%H%M%SZ").unwrap(),
            "us-east-1",
            &canonical_request,
        );

        let signing_key = get_signing_key(
            SECRET_KEY,
            chrono::NaiveDateTime::parse_from_str("20130524T000000Z", "%Y%m%dT%H%M%SZ").unwrap(),
            "us-east-1",
        );

        let signature = get_signature(signing_key, &string_to_sign);

        assert_eq!(
            signature,
            "34b48302e7b5fa45bde8084f4b7868a86f0a534bc59db6670ed5711ef69dc6f7"
        );
    }
}
