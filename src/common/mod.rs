use chacha20poly1305::aead::Aead;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};
use hkdf::Hkdf;
use sha2::Sha256;

pub const NONCE_LEN: usize = 12;

pub fn derive_key(password: &str) -> Key {
    let hkdf = Hkdf::<Sha256>::new(None, &[]);
    let mut key_bytes = [0u8; 32];
    hkdf.expand(password.as_bytes(), &mut key_bytes).unwrap();
    *Key::from_slice(&key_bytes)
}

pub fn encrypt_data(
    plaintext: &[u8],
    key: &Key,
    nonce: &Nonce,
) -> Result<Vec<u8>, chacha20poly1305::aead::Error> {
    let cipher = ChaCha20Poly1305::new(key);
    cipher.encrypt(nonce, plaintext)
}

pub fn decrypt_data(
    ciphertext: &[u8],
    key: &Key,
    nonce: &Nonce,
) -> Result<Vec<u8>, chacha20poly1305::aead::Error> {
    let cipher = ChaCha20Poly1305::new(key);
    cipher.decrypt(nonce, ciphertext)
}
