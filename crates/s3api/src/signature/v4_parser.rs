use s3err::ApiErrorCode;

use crate::datatype;

pub struct CredentialHeader {
    pub access_key: String,
    pub scope: Scope,
}

pub struct Scope {
    pub date: chrono::NaiveDate,
    pub region: String,
    pub service: String,
    pub request: String,
}

pub fn parse_credential(s: &String) -> Result<CredentialHeader, ApiErrorCode> {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() != 5 {
        return Err(s3err::ApiErrorCode::ErrCredMalformed);
    }
    // check if valid access key

    let mut cred = CredentialHeader {
        access_key: parts[0].to_string(),
        scope: Scope {
            date: chrono::NaiveDate::MIN,
            region: parts[2].to_string(),
            service: parts[3].to_string(),
            request: parts[4].to_string(),
        },
    };
    let date = chrono::NaiveDate::parse_from_str(parts[1], datatype::YYYY_MM_DD)
        .map_err(|_| s3err::ApiErrorCode::ErrMalformedDate)?;
    cred.scope.date = date;
    if cred.scope.region == "" {
        return Err(s3err::ApiErrorCode::ErrInvalidRegion);
    }
    if cred.scope.service != "s3" {
        return Err(s3err::ApiErrorCode::ErrInvalidService);
    }
    if cred.scope.request != "aws4_request" {
        return Err(s3err::ApiErrorCode::ErrInvalidRequestVersion);
    }
    Ok(cred)
}
