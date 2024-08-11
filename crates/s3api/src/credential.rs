use aws_sdk_s3::types::SessionCredentials;

#[derive(Debug)]
pub struct Credential {
    pub user_id: String,
    pub display_name: String,
    pub allow_other_user_access: bool,
    session_credentials: SessionCredentials,
}

impl Credential {
    pub fn to_string(&self) -> String {
        format!(
            "UserId: {}, AccessKey: {}, SecretKey: {}",
            self.user_id,
            self.session_credentials.access_key_id,
            self.session_credentials.secret_access_key
        )
    }
}
