use sha1::{Digest, Sha1};

pub fn hex_to_bytes(hex: &str) -> [u8; 20] {
    let mut bytes = [0u8; 20];
    for i in 0..20 {
        bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap();
    }
    bytes
}

pub fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
