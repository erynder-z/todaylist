//! Utilities for date and time formatting.

use chrono::Local;

/// Returns the current local date formatted as `YYYY-MM-DD`.
pub fn get_current_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}
