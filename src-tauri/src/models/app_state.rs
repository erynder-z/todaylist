use crate::models::note_session::NoteSession;
use crate::services::note_manager::NoteManager;
use std::sync::Mutex;

pub struct AppState {
    pub note_manager: Mutex<NoteManager>,
    pub note_session: Mutex<NoteSession>,
}
