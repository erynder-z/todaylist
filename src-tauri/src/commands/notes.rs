//! Tauri commands for note-related operations.
//!
//! This module provides functions for reading, writing, and manipulating note files, as well as managing the current note editing session.

use crate::models::app_state::AppState;
use crate::models::response_types::{
    FormattedNote, NoteContentResponse, NoteMetadata, SearchResult,
};
use std::fs;
use std::path::PathBuf;
use tauri::State;

/// Saves the complete content of a note to the specified path.
#[tauri::command]
pub async fn save_note_content(
    path: String,
    content: String,
    _state: State<'_, AppState>,
) -> Result<(), String> {
    fs::write(PathBuf::from(path), content).map_err(|e| format!("Failed to save note: {}", e))
}

/// Updates the content of a specific line in the current note session.
///
/// This operation also writes the entire note to disk.
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

/// Inserts a new line into the current note session at the specified index.
///
/// This operation also writes the entire note to disk.
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

/// Deletes the line at the specified index from the current note session.
///
/// This operation also writes the entire note to disk.
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

/// Adds a tag to the current note session and writes it to disk.
///
/// Returns the updated `NoteContentResponse`.
#[tauri::command]
pub async fn add_note_tag(
    tag: String,
    state: State<'_, AppState>,
) -> Result<NoteContentResponse, String> {
    let mut session = state.note_session.lock().unwrap();
    session.add_tag(tag);

    let path = session
        .path
        .clone()
        .ok_or_else(|| "No active note session".to_string())?;
    let full_content = session.get_full_content();
    fs::write(&path, full_content).map_err(|e| format!("Failed to save note: {}", e))?;

    let formatted_date = {
        let note_manager = state.note_manager.lock().unwrap();
        let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
        note_manager.format_note_name(filename)
    };

    let tags = session.get_tags();
    let raw_metadata = session.get_metadata();

    Ok(NoteContentResponse {
        lines: session.lines.clone(),
        metadata: NoteMetadata {
            formatted_date,
            tags,
            raw: raw_metadata,
        },
        metadata_range: session.frontmatter_range,
    })
}

/// Performs a full-text search across all notes. (Currently not implemented)
#[tauri::command]
pub async fn search_notes(_query: String) -> Result<Vec<SearchResult>, String> {
    // TODO: Implement note search
    Ok(vec![])
}

/// Returns the absolute path to today's daily note.
#[tauri::command]
pub async fn get_today_note_path(state: State<'_, AppState>) -> Result<String, String> {
    let note_manager = state.note_manager.lock().unwrap();
    note_manager.ensure_notes_folder_exists()?;
    let file_path = note_manager.get_today_note_path();
    Ok(file_path.to_string_lossy().into_owned())
}

/// Checks if today's daily note file already exists.
#[tauri::command]
pub async fn check_todays_note_exists(state: State<'_, AppState>) -> Result<bool, String> {
    let note_manager = state.note_manager.lock().unwrap();
    let file_path = note_manager.get_today_note_path();
    Ok(file_path.exists())
}

/// Creates a new daily note for today if it doesn't already exist.
///
/// Automatically initializes the note with a localized header.
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

/// Reads the content of a note file from the specified path.
///
/// If the note file does not exist, returns an error.
///
/// Automatically loads the note content into the application session.
///
/// Returns a `NoteContentResponse` containing the note's content and metadata.
#[tauri::command]
pub async fn read_note_content(
    path: String,
    state: State<'_, AppState>,
) -> Result<NoteContentResponse, String> {
    let path_buf = PathBuf::from(&path);
    let content = {
        let note_manager = state.note_manager.lock().unwrap();
        note_manager.read_note_content(&path_buf)?
    };

    let mut session = state.note_session.lock().unwrap();
    session.load(path_buf.clone(), content);

    let formatted_date = {
        let note_manager = state.note_manager.lock().unwrap();
        let filename = path_buf.file_name().and_then(|f| f.to_str()).unwrap_or("");
        note_manager.format_note_name(filename)
    };

    let tags = session.get_tags();
    let raw_metadata = session.get_metadata();

    Ok(NoteContentResponse {
        lines: session.lines.clone(),
        metadata: NoteMetadata {
            formatted_date,
            tags,
            raw: raw_metadata,
        },
        metadata_range: session.frontmatter_range,
    })
}

/// Returns a list of all notes available in the current notes folder.
#[tauri::command]
pub async fn list_notes(state: State<'_, AppState>) -> Result<Vec<FormattedNote>, String> {
    let note_manager = state.note_manager.lock().unwrap();
    note_manager.list_notes()
}

/// Returns all unique tags from all notes, sorted by usage frequency.
#[tauri::command]
pub async fn get_all_tags(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let note_manager = state.note_manager.lock().unwrap();
    note_manager.get_all_tags()
}
