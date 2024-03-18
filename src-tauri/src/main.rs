#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use state::AppState;

mod auth;
mod crypto;
mod vault;
mod command;
mod state;

use crate::command::*;

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![list_items, encrypt_item, decrypt_item, remove_item])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
