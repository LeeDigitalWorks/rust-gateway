pub struct BucketLoggingStatus {
  pub logging_enabled: Option<BucketLoggingRule>,
}

impl BucketLoggingStatus {
  pub fn new() -> BucketLoggingStatus {
    BucketLoggingStatus {
      logging_enabled: None,
    }
  }
}

pub struct BucketLoggingRule {
  pub target_bucket: String,
  pub target_prefix: String,
}
