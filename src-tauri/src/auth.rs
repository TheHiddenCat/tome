use argon2::{password_hash::SaltString, Argon2};
use rand_core::OsRng;

pub fn generate_salt() -> String {
    let salt = SaltString::generate(&mut OsRng);
    salt.to_string()
}

pub fn derive_encryption_key(passphrase: &str, salt: &str) -> [u8; 32] {
    let mut key = [0u8; 32];
    let _ = Argon2::default().hash_password_into(passphrase.as_bytes(), salt.as_bytes(), &mut key);
    key
}