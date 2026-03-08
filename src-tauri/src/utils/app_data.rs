//! Utilities for application-specific data and directory paths.

use std::path::PathBuf;

/// Returns the user's home directory path.
/// Defaults to the current directory if it cannot be determined.
pub fn get_home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

/// Returns the absolute path to the application's local data directory.
///
/// This is where the configuration and potentially other persistent
/// app-related data is stored.
pub fn get_app_data_dir() -> PathBuf {
    get_home_dir().join(".todaynote")
}
