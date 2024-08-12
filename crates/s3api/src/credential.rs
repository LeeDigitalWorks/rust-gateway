use aws_sdk_s3::types::SessionCredentials;

#[derive(Debug, Default)]
pub struct Credential {
    pub user_id: String,
    pub display_name: String,
    pub allow_other_user_access: bool,
    pub session_credentials: Option<SessionCredentials>,
}

impl Credential {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "UserId: {}, AccessKey: {}, SecretKey: {}",
            self.user_id,
            self.session_credentials
                .as_ref()
                .map_or("", |x| &x.access_key_id),
            self.session_credentials
                .as_ref()
                .map_or("", |x| &x.secret_access_key),
        )
    }
}
