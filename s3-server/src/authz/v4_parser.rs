use axum::http::HeaderMap;

pub fn parse_sign_v4(auth_string: String, headers: &HeaderMap) -> Result<(), s3_core::S3Error> {
    let auth_string = auth_string.split(" ").collect::<Vec<&str>>()[1];
    let auth_string = auth_string.split(",").collect::<Vec<&str>>();

    let mut cred = "";
    let mut signed_headers = "";
    let mut signature = "";

    Ok(())
}
