use crate::models::config::AppConfig;
use crate::models::response_types::ConfigResponse;
use std::path::PathBuf;

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
