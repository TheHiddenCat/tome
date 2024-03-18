use std::path::Path;

use tauri::State;

use crate::{state::AppState, vault::Secret};

#[tauri::command]
pub fn list_items(state: State<AppState>) -> Vec<Secret>  {
    let vault = state.data.lock().unwrap();
    vault.list()
}

#[tauri::command]
pub fn encrypt_item(passphrase: &str, category: &str, key: &str, data: &str, state: State<AppState>) -> bool {
    let mut vault = state.data.lock().unwrap();
    vault.add(passphrase, category.to_owned(), key.to_owned(), data.to_owned());
    vault.persist(Path::new("./assets/data.tome")).is_ok()
}

#[tauri::command]
pub fn decrypt_item(passphrase: &str, category: &str, key: &str, state: State<AppState>) -> Option<String> {
    let vault = state.data.lock().unwrap();
    vault.get(passphrase, category, key)
}

#[tauri::command]
pub fn remove_item(key: &str, category: &str, state: State<AppState>) -> bool {
    let mut vault = state.data.lock().unwrap();
    vault.remove(category, key);
    vault.persist(Path::new("./assets/data.tome")).is_ok()
}