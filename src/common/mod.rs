use aes_gcm::Nonce;
use aes_gcm::{
    aead::{Aead, Error},
    Aes256Gcm, KeyInit,
};

use sha2::{Digest, Sha256};

pub fn derive_key(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..32]);
    key
}

pub fn encrypt_data(data: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> Vec<u8> {
    let aead = Aes256Gcm::new_from_slice(key).expect("Failed to create AES256GCM");
    let nonce_data = Nonce::from_slice(nonce);
    let encrypted_data = aead
        .encrypt(nonce_data, data)
        .expect("Failed to encrypt data");
    encrypted_data
}

pub fn decrypt_data(data: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> Result<Vec<u8>, Error> {
    let aead = Aes256Gcm::new_from_slice(key).expect("Failed to create AES256GCM");
    let nonce_data = Nonce::from_slice(nonce);
    let decrypted_data = aead.decrypt(nonce_data, data)?;
    Ok(decrypted_data)
}

pub fn derive_nonce(password: &str) -> [u8; 12] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&result[..12]);
    nonce
}

pub fn increment_nonce(nonce: &mut [u8; 12]) {
    for i in (0..12).rev() {
        nonce[i] = nonce[i].wrapping_add(1);
        if nonce[i] != 0 {
            break;
        }
    }
}
