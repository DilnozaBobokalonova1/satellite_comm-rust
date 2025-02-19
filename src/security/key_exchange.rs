use blake2::{
    digest::{Update, VariableOutput},
    Blake2sVar,
};
use rand::rngs::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};

/**
 * StaticSecret type is identical to the EphemeralSecret type, except
 * that the StaticSecret::diffie_hellman method does not consume the
 * secret key, and the type provides serialization methods to save and
 * load key material. This means that the secret may be used multiple
 * times (but does not have to be).
 */
pub fn generate_keypair() -> (StaticSecret, PublicKey) {
    let private_key = StaticSecret::random_from_rng(OsRng);
    let public_key = PublicKey::from(&private_key);

    (private_key, public_key)
}

/**
 * Derive a shared secret using X25519 ECDH key exchange.
 */
pub fn derive_shared_secret(private_key: &StaticSecret, peer_public_key: &PublicKey) -> [u8; 32] {
    private_key.diffie_hellman(peer_public_key).to_bytes()
}

const DERIVED_KEY_SIZE: usize = 32; // 256-bit key for encryption

/**
 * Derive a high-entropy encryption key from the shared secret using Blake2b KDF.
 */
pub fn derive_encryption_key(shared_secret: &[u8]) -> Result<[u8; DERIVED_KEY_SIZE], &'static str> {
    let mut key = [0u8; DERIVED_KEY_SIZE];

    let mut kdf =
        Blake2sVar::new(DERIVED_KEY_SIZE).map_err(|_| "Invalid output size for Blake2b.")?;
    kdf.update(shared_secret);
    kdf.finalize_variable(&mut key)
        .map_err(|_| "KDF key derivation failed.")?;

    Ok(key)
}
