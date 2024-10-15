use axum::extract::Request;
use s3_core::S3Error;

const AWS2_PREFIX: &str = "AWS ";
const AWS4_PREFIX: &str = "AWS4";
const CREDENTIAL: &str = "Credential=";

enum AuthType {
    Unknown,
    Aws2Header,
    Aws2Query,
    Aws4Header,
    Aws4Query,
}

pub fn parse_access_key(req: &Request) -> Result<String, S3Error> {
    let mut auth_type = AuthType::Unknown;
    let auth_val = req.headers().get("Authorization");

    let mut header_auth = false;

    if auth_val.is_some() {
        header_auth = true;
        if let Some(auth_val) = auth_val {
            let auth_val = auth_val.to_str().unwrap();
            if auth_val.starts_with(AWS4_PREFIX) {
                auth_type = AuthType::Aws4Header;
            } else if auth_val.starts_with(AWS2_PREFIX) {
                auth_type = AuthType::Aws2Header;
            }
        }
    }

    match auth_type {
        AuthType::Aws4Header => {
            let key = run_aws4_header(auth_val.unwrap().to_str().unwrap());
            return Ok(key);
        }
        _ => Err(S3Error::InvalidRequest),
    }
}

fn run_aws4_header(auth_val: &str) -> String {
    // Authorization: AWS4-HMAC-SHA256
    // Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request,
    // SignedHeaders=host;range;x-amz-date,
    // Signature=fe5f80f77d5fa3beca038a248ff027d0445342fe2855ddc963176630326f1024
    let auth_val = auth_val.split(CREDENTIAL).collect::<Vec<&str>>()[1];
    extract_key_from_v4_credentials(auth_val)
}

fn extract_key_from_v4_credentials(creds: &str) -> String {
    let creds: Vec<&str> = creds.split('/').collect();
    creds[0].to_string()
}
