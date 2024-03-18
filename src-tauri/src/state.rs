use std::sync::Mutex;

use crate::vault::Vault;

pub struct AppState {
    pub data: Mutex<Vault>
}

impl AppState {
    pub fn new() -> Self {
        AppState { data: Mutex::new(Vault::load_or_new()) }
    }
}