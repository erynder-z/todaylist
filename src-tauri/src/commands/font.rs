//! Tauri commands for font management.

use crate::models::app_state::AppState;
use tauri::State;

/// Retrieves a list of available system fonts.
/// Returns a list of font family names that can be used in CSS.
#[tauri::command]
pub async fn get_system_fonts() -> Result<Vec<String>, String> {
    // Get the complete list of installed fonts for the current platform
    get_all_system_fonts()
}

/// Platform-specific function to get all installed system fonts
fn get_all_system_fonts() -> Result<Vec<String>, String> {
    #[cfg(target_os = "macos")]
    {
        get_all_macos_fonts()
    }

    #[cfg(target_os = "windows")]
    {
        get_all_windows_fonts()
    }

    #[cfg(target_os = "linux")]
    {
        get_all_linux_fonts()
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        // For other platforms, try macOS method first (might work on BSD), then fallback
        get_all_macos_fonts().or_else(|_| Ok(get_fallback_fonts()))
    }
}

#[cfg(target_os = "macos")]
fn get_all_macos_fonts() -> Result<Vec<String>, String> {
    use std::process::Command;

    // Method 1: Try using system_profiler to get all installed fonts
    let output = Command::new("system_profiler")
        .arg("SPFontsDataType")
        .output();

    match output {
        Ok(result) if result.status.success() => {
            let output_str = String::from_utf8_lossy(&result.stdout);
            let mut fonts = Vec::new();

            // Parse the output to extract ALL font names
            // The output contains lines like: "    Font: Font Name (Font Type)"
            for line in output_str.lines() {
                if line.trim().starts_with("Font:") {
                    if let Some(font_part) = line.split('(').next() {
                        let font_name = font_part.trim().trim_start_matches("Font: ").trim();
                        if !font_name.is_empty() {
                            fonts.push(font_name.to_string());
                        }
                    }
                }
            }

            // Sort and deduplicate
            fonts.sort();
            fonts.dedup();

            if fonts.is_empty() {
                // If system_profiler didn't return any fonts, try alternative method
                return get_macos_fonts_alternative();
            }

            Ok(fonts)
        }
        _ => {
            // Fallback to alternative method if system_profiler fails
            get_macos_fonts_alternative()
        }
    }
}

#[cfg(target_os = "macos")]
fn get_macos_fonts_alternative() -> Result<Vec<String>, String> {
    use std::fs;
    use std::path::Path;

    let mut fonts = Vec::new();

    // Common macOS font directories
    let font_dirs = [
        "/Library/Fonts",
        "/System/Library/Fonts",
        "/Network/Library/Fonts",
    ];

    // Also check user font directories
    if let Some(home) = std::env::var("HOME").ok() {
        let user_font_dirs = [format!("{}/Library/Fonts", home)];

        for dir in user_font_dirs {
            if Path::new(&dir).exists() {
                if let Ok(entries) = fs::read_dir(&dir) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Some(name) = entry.file_name().to_str() {
                                // Extract font family name from filename
                                if let Some(family) = extract_font_family_from_filename(name) {
                                    fonts.push(family);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Check system font directories
    for dir in font_dirs {
        if Path::new(dir).exists() {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(name) = entry.file_name().to_str() {
                            if let Some(family) = extract_font_family_from_filename(name) {
                                fonts.push(family);
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort and deduplicate
    fonts.sort();
    fonts.dedup();

    if fonts.is_empty() {
        // If we still have no fonts, return fallback
        Ok(get_fallback_fonts())
    } else {
        Ok(fonts)
    }
}

#[cfg(target_os = "windows")]
fn get_all_windows_fonts() -> Result<Vec<String>, String> {
    use std::process::Command;

    // Method 1: Try using PowerShell to get installed fonts
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-WmiObject -Class Win32_LogicalFont | Select-Object -ExpandProperty Name")
        .output();

    match output {
        Ok(result) if result.status.success() => {
            let output_str = String::from_utf8_lossy(&result.stdout);
            let mut fonts = Vec::new();

            for line in output_str.lines() {
                let font_name = line.trim().to_string();
                if !font_name.is_empty() {
                    fonts.push(font_name);
                }
            }

            if !fonts.is_empty() {
                fonts.sort();
                fonts.dedup();
                return Ok(fonts);
            }
        }
        _ => {}
    }

    // Method 2: Try using reg query to get fonts from registry
    let output = Command::new("reg")
        .arg("query")
        .arg("HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Fonts")
        .output();

    match output {
        Ok(result) if result.status.success() => {
            let output_str = String::from_utf8_lossy(&result.stdout);
            let mut fonts = Vec::new();

            // Parse registry output to extract font names
            for line in output_str.lines().skip(1) {
                // Skip header line
                if let Some(font_name) = line.split_whitespace().next() {
                    // Remove quotes and .ttf/.otf extensions
                    let clean_name = font_name
                        .trim_matches('"')
                        .trim_end_matches(".ttf")
                        .trim_end_matches(".otf")
                        .trim();
                    if !clean_name.is_empty() {
                        fonts.push(clean_name.to_string());
                    }
                }
            }

            if !fonts.is_empty() {
                fonts.sort();
                fonts.dedup();
                return Ok(fonts);
            }
        }
        _ => {}
    }

    // Method 3: Fallback to reading C:\Windows\Fonts directory
    let fonts_dir = "C:\\Windows\\Fonts";
    if let Ok(entries) = std::fs::read_dir(fonts_dir) {
        let mut fonts = Vec::new();

        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if let Some(family) = extract_font_family_from_filename(name) {
                        fonts.push(family);
                    }
                }
            }
        }

        if !fonts.is_empty() {
            fonts.sort();
            fonts.dedup();
            return Ok(fonts);
        }
    }

    // Final fallback
    Ok(get_fallback_fonts())
}

#[cfg(target_os = "linux")]
fn get_all_linux_fonts() -> Result<Vec<String>, String> {
    use std::process::Command;

    let output = Command::new("fc-list")
        .args(["--format=%{family}\\n"])
        .output()
        .map_err(|e| format!("Failed to execute fc-list: {e}"))?;

    let output_str = String::from_utf8_lossy(&output.stdout);

    let mut fonts = output_str
        .lines()
        .flat_map(|line| line.split(','))
        .map(str::trim)
        .filter(|font| !font.is_empty())
        .map(String::from)
        .collect::<Vec<_>>();

    fonts.sort();
    fonts.dedup();

    if fonts.is_empty() {
        Ok(get_fallback_fonts())
    } else {
        Ok(fonts)
    }
}

/// Recursively scan a font directory and extract font family names
#[cfg(target_os = "linux")]
fn scan_font_directory(dir: &str, fonts: &mut Vec<String>) {
    use std::fs;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    // Recursively scan subdirectories
                    if let Some(dir_str) = path.to_str() {
                        scan_font_directory(dir_str, fonts);
                    }
                } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // Extract font family from filename
                    if let Some(family) = extract_font_family_from_filename(name) {
                        fonts.push(family);
                    }
                }
            }
        }
    }
}

/// Extract font family name from a filename
/// Handles cases like "Arial.ttf", "Arial Bold.ttf", "Arial_Bold_Italic.ttf"
fn extract_font_family_from_filename(filename: &str) -> Option<String> {
    // Remove file extension
    let name = filename.split('.').next()?;

    // Handle common naming patterns
    let clean_name = name
        .replace("_", " ")  // Replace underscores with spaces
        .replace("-", " ")  // Replace hyphens with spaces
        ;

    // Extract base family name (remove style suffixes like "Bold", "Italic", etc.)
    let base_name = clean_name
        .trim_end_matches(" Bold Italic")
        .trim_end_matches(" Italic Bold")
        .trim_end_matches(" Bold")
        .trim_end_matches(" Italic")
        .trim_end_matches(" Regular")
        .trim_end_matches(" Normal")
        .trim_end_matches(" Light")
        .trim_end_matches(" Medium")
        .trim_end_matches(" Thin")
        .trim_end_matches(" Black")
        .trim_end_matches(" ExtraBold")
        .trim_end_matches(" SemiBold")
        .trim();

    if base_name.is_empty() {
        None
    } else {
        Some(base_name.to_string())
    }
}

fn get_fallback_fonts() -> Vec<String> {
    // Common web-safe fonts as fallback
    vec![
        "Arial".to_string(),
        "Arial Black".to_string(),
        "Comic Sans MS".to_string(),
        "Courier New".to_string(),
        "Georgia".to_string(),
        "Impact".to_string(),
        "Lucida Console".to_string(),
        "Lucida Sans Unicode".to_string(),
        "Palatino Linotype".to_string(),
        "Tahoma".to_string(),
        "Times New Roman".to_string(),
        "Trebuchet MS".to_string(),
        "Verdana".to_string(),
        "Microsoft Sans Serif".to_string(),
        "Microsoft Serif".to_string(),
        "Monaco".to_string(),
        "Menlo".to_string(),
        "Consolas".to_string(),
        "Ubuntu".to_string(),
        "Roboto".to_string(),
        "Open Sans".to_string(),
        "Lato".to_string(),
        "Source Sans Pro".to_string(),
        "Helvetica".to_string(),
        "Geneva".to_string(),
        "San Francisco".to_string(),
        "Segoe UI".to_string(),
        "DejaVu Sans".to_string(),
        "DejaVu Serif".to_string(),
        "DejaVu Sans Mono".to_string(),
        "Liberation Sans".to_string(),
        "Liberation Serif".to_string(),
        "Liberation Mono".to_string(),
    ]
}

/// Sets the application font family.
#[tauri::command]
pub async fn set_font_family(
    font_family: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut config = state.config()?;
    config.font_family = Some(font_family);
    config.save();

    Ok(())
}

/// Sets whether to use custom font or default app font.
#[tauri::command]
pub async fn set_use_custom_font(
    use_custom: bool,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut config = state.config()?;
    config.use_custom_font = use_custom;
    config.save();

    Ok(())
}
