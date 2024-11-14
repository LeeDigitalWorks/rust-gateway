// Maximum object part size for multipart upload
pub static MAX_OBJECT_PART_SIZE: usize = 5 * 1024 * 1024 * 1024; // 5GB

// https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html
pub fn is_valid_bucket_name(b: &str) -> bool {
    let len = b.len();
    if len < 3 || len > 63 {
        return false;
    }

    if b.contains("..") {
        return false;
    }

    if b.contains("-.") || b.contains(".-") {
        return false;
    }

    if b.starts_with('-') || b.ends_with('-') {
        return false;
    }

    if b.starts_with('.') || b.ends_with('.') {
        return false;
    }

    if b.chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '.')
        .count()
        != len
    {
        return false;
    }

    true
}
