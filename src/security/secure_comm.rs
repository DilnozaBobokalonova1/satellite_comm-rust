use ed25519_dalek::{Signature, SigningKey, VerifyingKey};
use x25519_dalek::{PublicKey, StaticSecret};

use super::{
    encryption::{self, XCHACHA20_POLY1305_NONCE_SIZE},
    key_exchange, signature,
};

pub struct SignedAndEncryptedMessage {
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; XCHACHA20_POLY1305_NONCE_SIZE],
    pub ephemeral_public_key: [u8; 32],
    pub signature: Signature,
}

pub fn encrypt_and_sign(
    plaintext: &str,
    sender_signing_key: &mut SigningKey,
    receiver_public_key: &PublicKey,
) -> Result<SignedAndEncryptedMessage, &'static str> {
    // First, we generate an ephemeral keypair from X25519 that uses OS randomness underneath
    let (ephemeral_private_key, ephemeral_public_key) = key_exchange::generate_keypair();

    // Then, we compute a shared secret using X25519 ECDH based on ephemeral private key and receiver's public key that the sender has
    let shared_secret =
        key_exchange::derive_shared_secret(&ephemeral_private_key, receiver_public_key);
    // Derive encryption key from shared secret using Blake2b
    let encryption_key = key_exchange::derive_encryption_key(&shared_secret).expect("KDF Failure.");

    // Now, we encrypt the message using Xchacha20
    let (ciphertext, nonce) = encryption::encrypt_message(plaintext, &encryption_key, None)
        .expect("Encryption of plaintext failed.");

    // And finally, we sign the ciphertext using Ed25519
    let signature = signature::sign_message(sender_signing_key, &ciphertext);

    Ok(SignedAndEncryptedMessage {
        ciphertext,
        nonce,
        ephemeral_public_key: ephemeral_public_key.as_bytes().to_owned(), // sendder's public key for key agreement
        signature,
    })
}

pub fn verify_and_decrypt(
    signed_encrypted_msg: &SignedAndEncryptedMessage,
    sender_verifying_key: &VerifyingKey,
    receiver_private_key: &StaticSecret,
) -> Result<String, &'static str> {
    let sender_ephemeral_public_key = PublicKey::from(signed_encrypted_msg.ephemeral_public_key);
    let shared_secret =
        key_exchange::derive_shared_secret(receiver_private_key, &sender_ephemeral_public_key);

    // Gen encryption key using KDF
    let encryption_key = key_exchange::derive_encryption_key(&shared_secret).unwrap();
    let sender_signature = signed_encrypted_msg.signature;
    let ciphertext: &[u8] = signed_encrypted_msg.ciphertext.as_slice();
    let nonce = signed_encrypted_msg.nonce;
    match signature::verify_signature(sender_verifying_key, ciphertext, &sender_signature) {
        Ok(_) => Ok(
            encryption::decrypt_message(ciphertext, &nonce, &encryption_key, None)
                .map_err(|_| "Error decrypting message.")
                .unwrap(),
        ),
        Err(_) => Err("Error verifying signature."),
    }
}
