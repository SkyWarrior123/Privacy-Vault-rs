use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit, OsRng, generic_array::GenericArray}
};
use rand::RngCore;

use crate::error::VaultError;

pub struct Crypto {
    cipher: Aes256Gcm,
}

impl Crypto {
    pub fn new(key_bytes: &[u8]) -> Result<Self, VaultError> {
        let mut key = [0u8; 32];
        if key_bytes.len() < 32 {
            return Err(VaultError::Crypto("encryption key must be >= 32 bytes".into()));
        }
        key.copy_from_slice(&key_bytes[..32]);
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
        Ok(Self { cipher })
    }

    pub fn encrypt(&self, plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>, VaultError> {
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = GenericArray::from_slice(&nonce_bytes);

        let mut ciphertext = self
            .cipher
            .encrypt(nonce, aes_gcm::aead::Payload { msg: plaintext, aad })
            .map_err(|e| VaultError::Crypto(format!("encrypt failed: {e}")))?;

        let mut out = nonce_bytes.to_vec();
        out.append(&mut ciphertext);
        Ok(out)
    }

    pub fn decrypt(&self, data: &[u8], aad: &[u8]) -> Result<Vec<u8>, VaultError> {
        if data.len() < 12 {
            return Err(VaultError::Crypto("ciphertext too short".into()));
        }
        let (nonce_bytes, ct) = data.split_at(12);
        let nonce = GenericArray::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, aes_gcm::aead::Payload { msg: ct, aad })
            .map_err(|e| VaultError::Crypto(format!("decrypt failed: {e}")))?;

        Ok(plaintext)
    }
}