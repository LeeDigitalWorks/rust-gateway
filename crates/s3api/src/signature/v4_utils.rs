use hmac::{Hmac, Mac};

pub const UNSIGNED_PAYLOAD: &str = "UNSIGNED-PAYLOAD";

pub type HmacSha256 = Hmac<sha2::Sha256>;

pub fn sum_hmac(key: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let mut hmac = HmacSha256::new_from_slice(&key).expect("Invalid Key length");
    hmac.update(&data);
    hmac.finalize().into_bytes().to_vec()
}

