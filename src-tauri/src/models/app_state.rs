//! Application-wide state management for the Tauri backend.

use crate::models::note_session::NoteSession;
use crate::services::note_manager::NoteManager;
use std::sync::Mutex;

/// Global state accessible to all Tauri command handlers.
///
/// This struct is wrapped in a `tauri::State` and manages shared access
/// to services and transient session data via `Mutex` for thread safety.
pub struct AppState {
    /// Service for managing note files and directories.
    pub note_manager: Mutex<NoteManager>,
    /// Transient state for the currently active editing session.
    pub note_session: Mutex<NoteSession>,
}
