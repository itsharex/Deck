use std::path::Path;

use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::Sha256;
use tokio::fs;
use tracing::debug;

use crate::error::DeckError;

type HmacSha256 = Hmac<Sha256>;

/// Read the auth token from the token file.
pub async fn read_token(path: &Path) -> Result<String, DeckError> {
    if !path.exists() {
        return Err(DeckError::TokenNotFound {
            path: path.display().to_string(),
        });
    }
    let content = fs::read_to_string(path).await.map_err(|e| {
        DeckError::Auth(format!("无法读取 token 文件 {}: {}", path.display(), e))
    })?;
    let token = content.trim().to_string();
    if token.is_empty() {
        return Err(DeckError::Auth("token 文件为空".into()));
    }
    debug!("token loaded from {}", path.display());
    Ok(token)
}

/// Generate a random hex nonce (16 bytes → 32 hex chars).
pub fn generate_nonce() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 16] = rng.gen();
    hex::encode(bytes)
}

/// Current unix timestamp in seconds.
pub fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock before epoch")
        .as_secs()
}

/// Compute HMAC-SHA256 signature for a request.
///
/// Signing material: `{timestamp}|{nonce}|{body}`
pub fn sign_request(session_key: &str, timestamp: u64, nonce: &str, body: &[u8]) -> String {
    let mut mac =
        HmacSha256::new_from_slice(session_key.as_bytes()).expect("HMAC key can be any length");
    mac.update(timestamp.to_string().as_bytes());
    mac.update(b"|");
    mac.update(nonce.as_bytes());
    mac.update(b"|");
    mac.update(body);
    hex::encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonce_length() {
        let nonce = generate_nonce();
        assert_eq!(nonce.len(), 32);
    }

    #[test]
    fn sign_deterministic() {
        let sig1 = sign_request("secret", 1000, "abc", b"hello");
        let sig2 = sign_request("secret", 1000, "abc", b"hello");
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn sign_differs_on_nonce() {
        let sig1 = sign_request("secret", 1000, "aaa", b"hello");
        let sig2 = sign_request("secret", 1000, "bbb", b"hello");
        assert_ne!(sig1, sig2);
    }
}
