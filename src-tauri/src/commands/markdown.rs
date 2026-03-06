use pulldown_cmark::{html, Options, Parser};

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
