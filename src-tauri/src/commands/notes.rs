use crate::models::app_state::AppState;
use crate::models::response_types::{FormattedNote, SearchResult};
use std::fs;
use std::path::PathBuf;
use tauri::State;

#[tauri::command]
pub async fn save_note_content(
    path: String,
    content: String,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    fs::write(PathBuf::from(path), content).map_err(|e| format!("Failed to save note: {}", e))
}

#[tauri::command]
pub async fn update_note_line(
    index: usize,
    content: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut session = state.note_session.lock().unwrap();
    session.update_line(index, content);

    if let Some(path) = &session.path {
        let full_content = session.get_full_content();
        fs::write(path, full_content).map_err(|e| format!("Failed to save note: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn insert_note_line(
    index: usize,
    content: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut session = state.note_session.lock().unwrap();
    session.insert_line(index, content);

    if let Some(path) = &session.path {
        let full_content = session.get_full_content();
        fs::write(path, full_content).map_err(|e| format!("Failed to save note: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn delete_note_line(index: usize, state: State<'_, AppState>) -> Result<(), String> {
    let mut session = state.note_session.lock().unwrap();
    session.delete_line(index);

    if let Some(path) = &session.path {
        let full_content = session.get_full_content();
        fs::write(path, full_content).map_err(|e| format!("Failed to save note: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn search_notes(_query: String) -> Result<Vec<SearchResult>, String> {
    // TODO: Implement note search
    Ok(vec![])
}

#[tauri::command]
pub async fn get_today_note_path(state: State<'_, AppState>) -> Result<String, String> {
    let note_manager = state.note_manager.lock().unwrap();
    note_manager.ensure_notes_folder_exists()?;
    let file_path = note_manager.get_today_note_path();
    Ok(file_path.to_string_lossy().into_owned())
}

#[tauri::command]
pub async fn check_todays_note_exists(state: State<'_, AppState>) -> Result<bool, String> {
    let note_manager = state.note_manager.lock().unwrap();
    let file_path = note_manager.get_today_note_path();
    Ok(file_path.exists())
}

#[tauri::command]
pub async fn create_todays_note(path: String, state: State<'_, AppState>) -> Result<(), String> {
    let file_path = PathBuf::from(path);

    if file_path.exists() {
        return Ok(());
    }

    let note_manager = state.note_manager.lock().unwrap();
    let translations = crate::commands::i18n::get_translations(note_manager.locale.clone());
    let note_header = translations
        .get("note.header")
        .map(|s| s.as_str())
        .unwrap_or("Note");

    let created_path = note_manager.create_todays_note(note_header)?;

    // Load into session so auto-save works immediately
    if let Ok(content) = note_manager.read_note_content(&created_path) {
        let mut session = state.note_session.lock().unwrap();
        session.load(created_path, content);
    }

    Ok(())
}

#[tauri::command]
pub async fn read_note_content(path: String, state: State<'_, AppState>) -> Result<String, String> {
    let content = {
        let note_manager = state.note_manager.lock().unwrap();
        note_manager.read_note_content(&PathBuf::from(&path))?
    };

    let mut session = state.note_session.lock().unwrap();
    session.load(PathBuf::from(path), content.clone());

    Ok(content)
}

#[tauri::command]
pub async fn list_notes(state: State<'_, AppState>) -> Result<Vec<FormattedNote>, String> {
    let note_manager = state.note_manager.lock().unwrap();
    note_manager.list_notes()
}
