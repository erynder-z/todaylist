use crate::models::config::AppConfig;
use crate::utils::date::get_current_date;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub notes_folder: String,
}

#[tauri::command]
pub async fn get_config() -> Result<ConfigResponse, String> {
    let config = AppConfig::load();
    Ok(ConfigResponse {
        notes_folder: config.notes_folder.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
pub async fn set_notes_folder(path: String) -> Result<(), String> {
    let mut config = AppConfig::load();
    config.notes_folder = PathBuf::from(path);
    config.save();
    Ok(())
}

#[tauri::command]
pub async fn create_new_note() -> Result<String, String> {
    let config = AppConfig::load();
    let current_date = get_current_date();

    let mut counter = 1;
    loop {
        let file_name = format!("note{}.md", counter);
        let file_path = config.notes_folder.join(&file_name);

        if !file_path.exists() {
            let note_content = format!("# Note: {}\n\n", current_date);

            fs::write(&file_path, note_content)
                .map_err(|e| format!("Failed to create note: {}", e))?;

            return Ok(file_path.to_string_lossy().into_owned());
        }

        counter += 1;
    }
}
