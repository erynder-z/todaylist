// Main application module

mod commands;
mod models;
mod utils;

use commands::config::{create_new_note, get_config, set_notes_folder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            create_new_note,
            get_config,
            set_notes_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
