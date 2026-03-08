//! Utilities for managing the Tauri application window.

use tauri::{AppHandle, Manager, PhysicalSize};

/// Tauri command to make the main window visible.
///
/// This is typically called after the frontend is fully initialized
/// to avoid showing an empty window.
#[tauri::command]
pub async fn show_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Configures the initial size and centers the main window.
///
/// This is only called when "Remember Window Size" is disabled.
/// It attempts to set a balanced aspect ratio (4:3) based on
/// the current monitor's dimensions.
pub fn setup_main_window(app_handle: &AppHandle) -> Result<(), String> {
    let window = app_handle
        .get_webview_window("main")
        .ok_or_else(|| "Main window not found".to_string())?;

    if let Some(monitor) = window.current_monitor().map_err(|e| e.to_string())? {
        let monitor_size = monitor.size();

        // Default to 80% of monitor height and a 4:3 aspect ratio
        let mut height = (monitor_size.height as f64 * 0.8) as u32;
        let mut width = (height as f64 * 4.0 / 3.0) as u32;

        // If the calculated width is too wide for the monitor, cap at 80% width
        let max_width = (monitor_size.width as f64 * 0.8) as u32;
        if width > max_width {
            width = max_width;
            height = (width as f64 * 3.0 / 4.0) as u32;
        }

        window
            .set_size(PhysicalSize::new(width, height))
            .map_err(|e| format!("Failed to set window size: {}", e))?;

        window
            .center()
            .map_err(|e| format!("Failed to center window: {}", e))?;
    }

    Ok(())
}
