use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::{auth::{self, generate_salt}, crypto};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Secret {
    category: String,
    key: String,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vault {
    salt: String,
    data: Vec<Secret>
}

impl Secret {
    pub fn new(category: String, key: String, data: Vec<u8>) -> Self {
        Secret {
            category,
            key,
            data
        }
    }
}


impl Vault {
    pub fn new() -> Self {
        Vault {
            salt: generate_salt(),
            data: Vec::new(),
        }
    }

    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let data = std::fs::read(path)?;
        let storage: Vault = bincode::deserialize(&data)?;
        Ok(storage)
    }

    pub fn load_or_new() -> Self {
        if let Ok(a) = Vault::load(Path::new("./assets/data.tome")) {
            return a
        }
        Vault::new()
    }

    pub fn list(&self) -> Vec<Secret> {
        self.data.clone()
    }

    pub fn persist(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let data = bincode::serialize(&self)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn add(&mut self, passphrase: &str, category: String, key: String, data: String) {
        let encryption_key = auth::derive_encryption_key(passphrase, &self.salt);
        let data = crypto::encrypt(&encryption_key, data);
        let secret = Secret::new(category, key, data);
        self.data.push(secret);
    }

    pub fn get(&self, passphrase: &str, category: &str, key: &str) -> Option<String> {
        if let Some(secret) = self.data.iter().find(|secret| secret.category == category && secret.key == key) {
            let encryption_key = auth::derive_encryption_key(passphrase, &self.salt);
            return crypto::decrypt(&encryption_key, &secret.data);
        }
        None
    }

    pub fn remove(&mut self, category: &str, key: &str) {
        if let Some(index) = self.data.iter().position(|secret| secret.category == category && secret.key == key) {
            self.data.remove(index);
        }
    }
}