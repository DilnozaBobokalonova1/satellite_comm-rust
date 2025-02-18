/**
 * Why I chose XChaCha20 rather than AES-GCM:
 *  Resilient to nonce reuse attacks (no need for perfect randomness in nonce generation).
    Fast on low-power devices & embedded systems.
    Doesn’t need specialized hardware support (no AES-NI required).
    Prevents replay attacks (every message is unique).
 */

use chacha20poly1305::{aead::{Aead, KeyInit}, AeadCore, Key, XChaCha20Poly1305, XNonce};
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

pub fn encrypt_message(plaintext: &str, key: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = XChaCha20Poly1305::generate_nonce(&mut rand::thread_rng()); // 24-byte nonce
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes()).expect("Enryption failed.. Lets figure out why.");
    (ciphertext, nonce.to_vec())
}

pub fn decrypt_message(ciphertext: &[u8], nonce: &[u8], key: &[u8]) -> Option<String> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));

    match cipher.decrypt(XNonce::from_slice(nonce), ciphertext) {
        Ok(plaintext) => Some(String::from_utf8(plaintext).expect("Invalid UTF-8")),
        Err(_) => None, // add better Error handling
    }
}