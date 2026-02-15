use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub struct KeyPair {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}

impl KeyPair {
    /// Generate a new random Ed25519 keypair
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Sign a message (e.g., the document hash or body)
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }
}
