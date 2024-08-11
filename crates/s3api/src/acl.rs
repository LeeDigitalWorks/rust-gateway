use actix_web::http::header::HeaderMap;

use crate::datatype;

pub fn get_acl_from_header(headers: &HeaderMap) -> Result<datatype::Acl, anyhow::Error> {
  let header = headers.get("x-amz-acl");
  let mut acl = datatype::Acl::new();

  match header {
    Some(header) => {
      let acl_str = header.to_str()?;

      if !datatype::is_valid_canned_acl(acl_str) {
        return Err(anyhow::anyhow!("Invalid canned acl"));
      }

      acl.canned_acl = acl_str.to_string();
      Ok(acl)
    },
    None => Ok(acl)
  }
}