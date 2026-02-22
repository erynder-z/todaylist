use crate::models::config::AppConfig;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedNote {
    pub filename: String,
    pub formatted_name: String,
}

#[tauri::command]
pub async fn create_new_note() -> Result<String, String> {
    let config = AppConfig::load();
    let current_date = crate::utils::date::get_current_date();

    let mut counter = 1;
    loop {
        let file_name = format!("note{}.md", counter);
        let file_path = config.notes_folder.join(&file_name);

        if !file_path.exists() {
            let note_content = format!("# Note: {}", current_date);

            fs::write(&file_path, note_content)
                .map_err(|e| format!("Failed to create note: {}", e))?;

            return Ok(file_path.to_string_lossy().into_owned());
        }

        counter += 1;
    }
}

fn ensure_notes_folder_exists(config: &AppConfig) -> Result<(), String> {
    if !config.notes_folder.exists() {
        fs::create_dir_all(&config.notes_folder)
            .map_err(|e| format!("Failed to create notes folder: {}", e))?;
    }
    Ok(())
}

fn get_todays_note_path(config: &AppConfig, current_date: &str) -> PathBuf {
    let file_name = format!("{}.md", current_date);
    config.notes_folder.join(&file_name)
}

fn create_note(file_path: &PathBuf, note_content: &str) -> Result<(), String> {
    fs::write(file_path, note_content).map_err(|e| format!("Failed to create note: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn check_todays_note_exists() -> Result<bool, String> {
    let config = AppConfig::load();
    let current_date = crate::utils::date::get_current_date();

    ensure_notes_folder_exists(&config)?;
    let file_path = get_todays_note_path(&config, &current_date);

    Ok(file_path.exists())
}

#[tauri::command]
pub async fn create_todays_note() -> Result<String, String> {
    let config = AppConfig::load();
    let current_date = crate::utils::date::get_current_date();

    ensure_notes_folder_exists(&config)?;
    let file_path = get_todays_note_path(&config, &current_date);

    if file_path.exists() {
        return Err("Note for today already exists".to_string());
    }

    let note_content = format!("# Note: {}", current_date);
    create_note(&file_path, &note_content)?;

    Ok(file_path.to_string_lossy().into_owned())
}

fn format_note_name(note_name: &str) -> String {
    let without_ext = note_name.replace(".md", "");

    if let Ok(date) = NaiveDate::parse_from_str(&without_ext, "%Y-%m-%d") {
        format!("{}", date.format("%A, %B %e, %Y"))
    } else {
        without_ext
    }
}

#[tauri::command]
pub async fn list_notes() -> Result<Vec<FormattedNote>, String> {
    let config = AppConfig::load();

    let entries = fs::read_dir(&config.notes_folder)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut notes: Vec<FormattedNote> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let file_name = entry.file_name().into_string().ok()?;
            if file_name.ends_with(".md") && !file_name.starts_with(".") {
                Some(FormattedNote {
                    filename: file_name.clone(),
                    formatted_name: format_note_name(&file_name),
                })
            } else {
                None
            }
        })
        .collect();

    // Sort by filename (or by parsed date if you prefer)
    notes.sort_by(|a, b| a.filename.cmp(&b.filename));
    notes.reverse();

    Ok(notes)
}
