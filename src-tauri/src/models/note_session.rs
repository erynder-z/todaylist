//! Transient state for the active note being edited.

use std::path::PathBuf;

/// Represents an active note session, maintaining an in-memory
/// representation of the file's lines for real-time editing.
pub struct NoteSession {
    /// File path of the currently open note.
    pub path: Option<PathBuf>,
    /// Each line of the note as an individual string in the vector.
    pub lines: Vec<String>,
}

impl NoteSession {
    /// Creates a new, empty note session.
    pub fn new() -> Self {
        Self {
            path: None,
            lines: Vec::new(),
        }
    }

    /// Loads the content and path of a note into the session.
    pub fn load(&mut self, path: PathBuf, content: String) {
        self.path = Some(path);
        self.lines = content.split('\n').map(|s| s.to_string()).collect();
    }

    /// Replaces the content of a specific line in the session.
    pub fn update_line(&mut self, index: usize, content: String) {
        if index < self.lines.len() {
            self.lines[index] = content;
        }
    }

    /// Inserts a new line at the specified index.
    pub fn insert_line(&mut self, index: usize, content: String) {
        if index <= self.lines.len() {
            self.lines.insert(index, content);
        }
    }

    /// Removes a specific line from the session.
    ///
    /// At least one line is always preserved to avoid an empty vector.
    pub fn delete_line(&mut self, index: usize) {
        if index < self.lines.len() && self.lines.len() > 1 {
            self.lines.remove(index);
        }
    }

    /// Reconstructs the full note content by joining the lines with newlines.
    pub fn get_full_content(&self) -> String {
        self.lines.join("\n")
    }
}
