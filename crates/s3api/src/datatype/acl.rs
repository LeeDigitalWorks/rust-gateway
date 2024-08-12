pub const VALID_CANNED_ACL: [&str; 7] = [
    "private",
    "public-read",
    "public-read-write",
    "authenticated-read",
    "aws-exec-read",
    "bucket-owner-read",
    "bucket-owner-full-control",
];

pub struct Acl {
    pub canned_acl: String,
}

impl Acl {
    pub fn new() -> Self {
        Acl {
            canned_acl: "private".to_string(),
        }
    }
}

pub struct AccessControlPolicy {
    pub id: String,
    pub display_name: String,
    pub access_control_list: Vec<Grant>,
}

pub struct Grant {
    pub grantee: Grantee,
    pub permission: String,
}

pub struct Grantee {
    pub id: String,
    pub uri: String,
    pub display_name: String,
    pub email_address: String,
}

pub struct AccessControlPolicyResponse {
    pub id: String,
    pub display_name: String,
    pub access_control_list: Vec<GrantResponse>,
}

pub struct GrantResponse {
    pub grantee: GranteeResponse,
    pub permission: String,
}

pub struct GranteeResponse {
    pub id: String,
    pub uri: String,
    pub display_name: String,
    pub email_address: String,
}

pub fn is_valid_canned_acl(acl: &str) -> bool {
    VALID_CANNED_ACL.contains(&acl)
}
