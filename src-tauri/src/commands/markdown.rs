//! Tauri commands for Markdown processing.

use pulldown_cmark::{html, Options, Parser};

/// Renders a Markdown string into an HTML string.
///
/// This uses the `pulldown-cmark` library with several GFM-like options enabled,
/// such as tables and task lists.
#[tauri::command]
pub async fn render_markdown(markdown: String) -> Result<String, String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(&markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(html_output)
}
