use sha2::{Digest, Sha256};

pub fn version(content: &str) -> String {
    format!("{:x}", Sha256::digest(content.as_bytes()))
}
