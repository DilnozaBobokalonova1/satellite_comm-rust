use ed25519_dalek::ed25519::signature::SignerMut;
use ed25519_dalek::Signature;
use ed25519_dalek::SigningKey;
use ed25519_dalek::{Verifier, VerifyingKey};
use rand::rngs::OsRng;

/**
*  pub struct SigningKey {
       // The secret half of this signing key.
       pub(crate) secret_key: SecretKey,
       // The public half of this signing key.
       pub(crate) verifying_key: VerifyingKey,
   }
*/
pub fn generate_identity_keypair() -> (SigningKey, VerifyingKey) {
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let verifying_key: VerifyingKey = signing_key.verifying_key(); // getting public part of the key
    return (signing_key, verifying_key);
}

/// Sign the message using ED25519
pub fn sign_message(private_key: &mut SigningKey, message: &[u8]) -> Signature {
    private_key.sign(message)
}

pub fn verify_signature(
    public_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
) -> Result<(), ed25519_dalek::SignatureError> {
    // better later to return an actual error for signature fail.
    public_key.verify(message, signature)
}
