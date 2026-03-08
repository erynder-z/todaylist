//! Serializable data structures for frontend communication.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Serialized version of the application configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub notes_folder: String,
    pub locale: String,
    pub theme: String,
    pub remember_window_size: bool,
}

/// Metadata for a single note file in the list.
#[derive(Debug, Serialize, Deserialize)]
pub struct FormattedNote {
    /// Full filename on disk.
    pub filename: String,
    /// Human-readable, localized name of the note.
    pub formatted_name: String,
}

/// A search match from the note archive.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    /// Filename of the note containing the match.
    pub filename: String,
    /// A short text snippet showing the context of the match.
    pub excerpt: String,
}

/// The complete state payload sent to the frontend during initialization.
#[derive(Debug, Serialize, Deserialize)]
pub struct InitialAppState {
    pub notes_folder: Option<String>,
    pub locale: String,
    pub theme: String,
    pub remember_window_size: bool,
    pub available_locales: Vec<LocaleInfo>,
    pub available_themes: Vec<ThemeInfo>,
    pub translations: HashMap<String, String>,
    pub theme_colors: HashMap<String, String>,
    pub today_note_path: Option<String>,
    pub today_note_content: Option<String>,
}

/// Result of a folder validation check.
#[derive(Debug, Serialize, Deserialize)]
pub struct FolderValidation {
    /// Whether the path is valid for note storage.
    pub is_valid: bool,
    /// Whether the application has write permissions.
    pub is_writable: bool,
    /// Whether the folder already exists.
    pub exists: bool,
    /// Total number of notes found in the directory.
    pub note_count: usize,
    /// Detailed error message if `is_valid` is false.
    pub error: Option<String>,
}

/// Metadata for an available locale.
#[derive(Debug, Serialize, Deserialize)]
pub struct LocaleInfo {
    pub id: String,
    pub name: String,
}

/// Metadata for an available theme.
#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeInfo {
    pub id: String,
    pub name: String,
}
