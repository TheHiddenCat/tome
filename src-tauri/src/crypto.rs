use aes_gcm::{aead::Aead, AeadCore, Aes256Gcm, KeyInit};
use rand_core::OsRng;

const NONCE_OFFSET: usize = 12;

pub fn encrypt(encryption_key: &[u8; 32], data: String) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(encryption_key).unwrap();
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let encrypted_secret = cipher.encrypt(&nonce, data.as_bytes()).unwrap();
    let mut combined = nonce.to_vec();
    combined.extend(encrypted_secret);
    combined
}

pub fn decrypt(encryption_key: &[u8; 32], data: &[u8]) -> Option<String> {
    let cipher = Aes256Gcm::new_from_slice(encryption_key).unwrap();
    let (nonce, encrypted_secret) = data.split_at(NONCE_OFFSET);
    if let Ok(decrypted_secret) = cipher.decrypt(nonce.into(), encrypted_secret) {
        return Some(String::from_utf8(decrypted_secret).unwrap());
    }
    None
}
