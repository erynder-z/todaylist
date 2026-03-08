use std::path::PathBuf;

pub struct NoteSession {
    pub path: Option<PathBuf>,
    pub lines: Vec<String>,
}

impl NoteSession {
    pub fn new() -> Self {
        Self {
            path: None,
            lines: Vec::new(),
        }
    }

    pub fn load(&mut self, path: PathBuf, content: String) {
        self.path = Some(path);
        self.lines = content.split('\n').map(|s| s.to_string()).collect();
    }

    pub fn update_line(&mut self, index: usize, content: String) {
        if index < self.lines.len() {
            self.lines[index] = content;
        }
    }

    pub fn insert_line(&mut self, index: usize, content: String) {
        if index <= self.lines.len() {
            self.lines.insert(index, content);
        }
    }

    pub fn delete_line(&mut self, index: usize) {
        if index < self.lines.len() && self.lines.len() > 1 {
            self.lines.remove(index);
        }
    }

    pub fn get_full_content(&self) -> String {
        self.lines.join("\n")
    }
}
