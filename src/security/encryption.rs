/**
* Why I chose XChaCha20 rather than AES-GCM:
*  Resilient to nonce reuse attacks (no need for perfect randomness in nonce generation).
   Fast on low-power devices & embedded systems.
   Doesn’t need specialized hardware support (no AES-NI required).
   Prevents replay attacks (every message is unique).
*/
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    AeadCore, Key, XChaCha20Poly1305, XNonce,
};
use rand::Rng;

/**
 * Ensures high security (resistant to brute-force).
 */
pub const XCHACHA20_POLY1305_KEY_SIZE: usize = 32;
/**
 * XChaCha20 expands a 24-byte nonce into a full key stream,
 * reducing the risk of reusing a nonce.
 */
pub const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24;

// For testing only rn
pub fn generate_encryption_key() -> [u8; XCHACHA20_POLY1305_KEY_SIZE] {
    let mut key = [0u8; XCHACHA20_POLY1305_KEY_SIZE];
    rand::thread_rng().fill(&mut key);
    key
}

/**
 * Encrypt the message with optional Metadata AAD (additional associated data).
 */
pub fn encrypt_message(
    plaintext: &str,
    key: &[u8],
    associated_data: Option<&[u8]>,
) -> Result<(Vec<u8>, [u8; XCHACHA20_POLY1305_NONCE_SIZE]), &'static str> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = XChaCha20Poly1305::generate_nonce(&mut rand::thread_rng()); // 24-byte nonce

    let payload = Payload {
        msg: plaintext.as_bytes(),
        aad: associated_data.unwrap_or(&[]),
    };
    match cipher.encrypt(&nonce, payload) {
        Ok(ciphertext) => Ok((ciphertext, nonce.into())),
        Err(_) => Err("Encryption failed."),
    }
}

pub fn decrypt_message(
    ciphertext: &[u8],
    nonce: &[u8; XCHACHA20_POLY1305_NONCE_SIZE],
    key: &[u8],
    associated_data: Option<&[u8]>,
) -> Result<String, &'static str> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
    let payload = Payload {
        msg: ciphertext,
        aad: associated_data.unwrap_or(&[]),
    };

    // XNonce ensures that the nonce passed is exactly 24 bytes long (which is required by the cipher)
    match cipher.decrypt(XNonce::from_slice(nonce), payload) {
        Ok(plaintext) => Ok(String::from_utf8_lossy(&plaintext).to_string()),
        Err(_) => Err("Error while decrypting text."),
    }
}
