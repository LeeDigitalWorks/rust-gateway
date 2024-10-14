use hmac::{Hmac, Mac};
use md5::Digest;

pub type HmacSha256 = Hmac<sha2::Sha256>;

pub fn sum_hmac(key: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let mut hmac = HmacSha256::new_from_slice(&key).expect("Invalid Key length");
    hmac.update(&data);
    hmac.finalize().into_bytes().to_vec()
}

pub fn sum_md5(data: Vec<u8>) -> Vec<u8> {
    let mut hasher = md5::Md5::new();
    hasher.update(&data);
    hasher.finalize().to_vec()
}
